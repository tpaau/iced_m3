mod constants {
    use crate::animation::motion::Spring;

    pub const FAST_SPATIAL_STANDARD: Spring = Spring::new(0.9, 1400.0);
    pub const FAST_EFFECTS_STANDARD: Spring = Spring::new(1.0, 3800.0);
    pub const DEFAULT_SPATIAL_STANDARD: Spring = Spring::new(0.9, 700.0);
    pub const DEFAULT_EFFECTS_STANDARD: Spring = Spring::new(1.0, 1600.0);
    pub const SLOW_SPATIAL_STANDARD: Spring = Spring::new(0.9, 300.0);
    pub const SLOW_EFFECTS_STANDARD: Spring = Spring::new(1.0, 800.0);

    pub const FAST_SPATIAL_EXPRESSIVE: Spring = Spring::new(0.6, 800.0);
    pub const FAST_EFFECTS_EXPRESSIVE: Spring = Spring::new(1.0, 3800.0);
    pub const DEFAULT_SPATIAL_EXPRESSIVE: Spring = Spring::new(0.8, 380.0);
    pub const DEFAULT_EFFECTS_EXPRESSIVE: Spring = Spring::new(1.0, 1600.0);
    pub const SLOW_SPATIAL_EXPRESSIVE: Spring = Spring::new(0.8, 200.0);
    pub const SLOW_EFFECTS_EXPRESSIVE: Spring = Spring::new(1.0, 800.0);

    pub const POSITION_EPSILON: f32 = 0.001;
    pub const VELOCITY_EPSILON: f32 = 0.001;
}

use std::time::Instant;

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

use crate::animation::Interpolable;

pub fn fast_spatial(expressive: bool) -> Spring {
    match expressive {
        true => constants::FAST_SPATIAL_EXPRESSIVE,
        false => constants::FAST_SPATIAL_STANDARD,
    }
}

pub fn fast_effects(expressive: bool) -> Spring {
    match expressive {
        true => constants::FAST_EFFECTS_EXPRESSIVE,
        false => constants::FAST_EFFECTS_STANDARD,
    }
}

pub fn default_spatial(expressive: bool) -> Spring {
    match expressive {
        true => constants::DEFAULT_SPATIAL_EXPRESSIVE,
        false => constants::DEFAULT_SPATIAL_STANDARD,
    }
}

pub fn default_effects(expressive: bool) -> Spring {
    match expressive {
        true => constants::DEFAULT_EFFECTS_EXPRESSIVE,
        false => constants::DEFAULT_EFFECTS_STANDARD,
    }
}

pub fn slow_spatial(expressive: bool) -> Spring {
    match expressive {
        true => constants::SLOW_SPATIAL_EXPRESSIVE,
        false => constants::SLOW_SPATIAL_STANDARD,
    }
}

pub fn slow_effects(expressive: bool) -> Spring {
    match expressive {
        true => constants::SLOW_EFFECTS_EXPRESSIVE,
        false => constants::SLOW_EFFECTS_STANDARD,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub damping: f32,
    pub stiffness: f32,
}

impl Spring {
    pub const fn new(damping: f32, stiffness: f32) -> Self {
        Self { damping, stiffness }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpringMotion {
    pub spring: Spring,
    pub position: f32,
    pub velocity: f32,
    pub target: f32,
    pub time: Instant,
}

impl SpringMotion {
    pub const fn new(spring: Spring, target: f32, now: Instant) -> Self {
        Self {
            spring,
            position: 0.0,
            velocity: 0.0,
            target,
            time: now,
        }
    }

    pub fn step(&mut self, now: Instant) {
        let delta_time = now.duration_since(self.time);
        self.time = now;
        if self.is_at_rest() {
            return;
        }
        let zeta = self.spring.damping.max(0.0);
        let omega = self.spring.stiffness.max(0.0).sqrt();

        if omega == 0.0 {
            return;
        }

        // Solve relative to the target.
        let x0 = self.position - self.target;
        let v0 = self.velocity;

        let x;
        let v;

        if zeta < 1.0 {
            // Underdamped: produces overshoot.
            let decay = -zeta * omega;
            let frequency = omega * (1.0 - zeta * zeta).sqrt();

            let a = x0;
            let b = (v0 - decay * x0) / frequency;

            let envelope = (decay * delta_time.as_secs_f32()).exp();
            let angle = frequency * delta_time.as_secs_f32();
            let sin = angle.sin();
            let cos = angle.cos();

            x = envelope * (a * cos + b * sin);
            v = envelope * ((decay * a + frequency * b) * cos + (decay * b - frequency * a) * sin);
        } else if zeta == 1.0 {
            // Critically damped: fastest motion without overshoot.
            let envelope = (-omega * delta_time.as_secs_f32()).exp();
            let b = v0 + omega * x0;

            x = envelope * (x0 + b * delta_time.as_secs_f32());
            v = envelope * (v0 - omega * b * delta_time.as_secs_f32());
        } else {
            // Overdamped: slower motion without overshoot.
            let root = (zeta * zeta - 1.0).sqrt();
            let r1 = -omega * (zeta - root);
            let r2 = -omega * (zeta + root);

            let a = (v0 - r2 * x0) / (r1 - r2);
            let b = x0 - a;

            let e1 = (r1 * delta_time.as_secs_f32()).exp();
            let e2 = (r2 * delta_time.as_secs_f32()).exp();

            x = a * e1 + b * e2;
            v = a * r1 * e1 + b * r2 * e2;
        }

        self.position = self.target + x;
        self.velocity = v;
    }

    pub fn is_at_rest(&self) -> bool {
        (self.position - self.target).abs() <= constants::POSITION_EPSILON
            && self.velocity.abs() <= constants::VELOCITY_EPSILON
    }
}

pub struct ValueMotion<T>
where
    T: Interpolable + Clone + PartialEq,
{
    pub from: T,
    pub to: T,
    pub spring: SpringMotion,
}

impl<T> ValueMotion<T>
where
    T: Interpolable + Clone + PartialEq,
{
    pub const fn new(from: T, to: T, spring: Spring, now: Instant) -> Self {
        Self {
            from,
            to,
            spring: SpringMotion::new(spring, 0.0, now),
        }
    }

    pub fn value(&self) -> T {
        self.from
            .clone()
            .interpolate(self.to.clone(), self.spring.position)
    }

    pub fn set_target(&mut self, target: T, now: Instant) {
        if self.to == target {
            return;
        }

        self.from = self.value();
        self.to = target;

        self.spring.position = 0.0;
        self.spring.target = 1.0;
        self.spring.time = now;
    }

    pub fn step(&mut self, now: Instant) {
        self.spring.step(now);
    }

    pub fn is_at_rest(&self) -> bool {
        self.spring.is_at_rest()
    }
}
