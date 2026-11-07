use std::time::{Duration, Instant};

use iced::{Point, Size};

use super::constant::*;
use crate::animation::standard;

/// The pointer that owns the current press.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pointer {
    Mouse,
    Touch(u64),
}

/// Mirrors the official `State` enum of the ripple.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Phase {
    #[default]
    Inactive,
    TouchDelay {
        deadline: Instant,
    },
    Holding,
    WaitingForClick,
}

/// A linear opacity transition that follows CSS Transitions Level 1 §3,
/// including the faster reversing of interrupted transitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transition {
    from: f32,
    to: f32,
    reversing_adjusted_start: f32,
    reversing_shortening_factor: f32,
    started_at: Option<Instant>,
    duration: Duration,
}

impl Default for Transition {
    fn default() -> Self {
        Self::settled(0.0)
    }
}

impl Transition {
    #[must_use]
    pub fn settled(value: f32) -> Self {
        Self {
            from: value,
            to: value,
            reversing_adjusted_start: value,
            reversing_shortening_factor: 1.0,
            started_at: None,
            duration: Duration::ZERO,
        }
    }

    #[must_use]
    pub fn target(&self) -> f32 {
        self.to
    }

    /// Fraction of the (effective) duration that has elapsed, in `0..=1`.
    #[must_use]
    pub fn progress(&self, now: Instant) -> f32 {
        match self.started_at {
            Some(start) if !self.duration.is_zero() => {
                let elapsed = now.saturating_duration_since(start);
                (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
            }
            _ => 1.0,
        }
    }

    #[must_use]
    pub fn value(&self, now: Instant) -> f32 {
        let p = self.progress(now);
        self.from + (self.to - self.from) * p
    }

    #[must_use]
    pub fn is_running(&self, now: Instant) -> bool {
        self.started_at.is_some() && self.progress(now) < 1.0
    }

    /// Changes the end value, using `duration` as the specified duration.
    #[must_use]
    pub fn retarget(self, target: f32, duration: Duration, now: Instant) -> Self {
        if self.to == target {
            return self;
        }
        let current = self.value(now);
        if current == target || duration.is_zero() {
            return Self::settled(target);
        }

        let running = self.is_running(now);
        if running && self.reversing_adjusted_start == target {
            let factor = (self.progress(now) * self.reversing_shortening_factor
                + (1.0 - self.reversing_shortening_factor))
                .abs()
                .clamp(0.0, 1.0);
            return Self {
                from: current,
                to: target,
                reversing_adjusted_start: self.to,
                reversing_shortening_factor: factor,
                started_at: Some(now),
                duration: duration.mul_f32(factor),
            };
        }

        Self {
            from: current,
            to: target,
            reversing_adjusted_start: current,
            reversing_shortening_factor: 1.0,
            started_at: Some(now),
            duration,
        }
    }
}

/// Geometry of a press, fixed when it starts (the official ripple fixes the
/// animation keyframes at that point as well).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Press {
    pub start: Point,
    pub end: Point,
    pub initial_size: f32,
    pub scale: f32,
    pub started_at: Instant,
    pub pressed: bool,
    pub release_at: Option<Instant>,
}

impl Press {
    /// `determineRippleSize` of `ripple.ts:278-295`.
    #[must_use]
    pub fn new(origin: Option<Point>, size: Size, now: Instant) -> Self {
        let max_dim = size.width.max(size.height);
        let soft_edge = (SOFT_EDGE_CONTAINER_RATIO * max_dim).max(SOFT_EDGE_MINIMUM_SIZE);
        let initial_size = (max_dim * INITIAL_ORIGIN_SCALE).floor();
        let max_radius = size.width.hypot(size.height) + PADDING;
        let scale = (max_radius + soft_edge) / initial_size;
        let center = Point::new(size.width / 2.0, size.height / 2.0);

        Self {
            start: origin.unwrap_or(center),
            end: center,
            initial_size,
            scale,
            started_at: now,
            pressed: true,
            release_at: None,
        }
    }

    /// Eased grow progress in `0..=1`.
    #[must_use]
    pub fn grow(&self, now: Instant) -> f32 {
        let t =
            now.saturating_duration_since(self.started_at).as_secs_f32() / PRESS_GROW.as_secs_f32();
        standard(t.clamp(0.0, 1.0))
    }

    #[must_use]
    pub fn is_growing(&self, now: Instant) -> bool {
        now.saturating_duration_since(self.started_at) < PRESS_GROW
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Model {
    pub phase: Phase,
    pub pointer: Option<Pointer>,
    pub touch_origin: Option<Point>,
    pub hovered: bool,
    pub hover: Transition,
    pub fade: Transition,
    pub press: Option<Press>,
}

impl Model {
    #[must_use]
    pub fn is_animating(&self, now: Instant) -> bool {
        self.hover.is_running(now)
            || self.fade.is_running(now)
            || self.press.is_some_and(|press| press.is_growing(now))
    }

    /// The earliest instant at which a timer inside the model fires.
    #[must_use]
    pub fn next_deadline(&self) -> Option<Instant> {
        let touch = match self.phase {
            Phase::TouchDelay { deadline } => Some(deadline),
            _ => None,
        };
        let release = self
            .press
            .filter(|press| press.pressed)
            .and_then(|press| press.release_at);
        match (touch, release) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }
}
