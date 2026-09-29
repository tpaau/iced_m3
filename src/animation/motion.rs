//! Holds utilities related to material's [motion physics](https://m3.material.io/styles/motion/overview) system.

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

/// Defines the motion scheme used for motion physics.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scheme {
    /// The expressive scheme overshoots the final values to add bounce.
    ///
    /// This is the default with the `expressive-defaults` feature enabled.
    #[cfg_attr(feature = "expressive-defaults", default)]
    Expressive,
    /// The standard scheme eases into the final values and feels more utilitarian.
    ///
    /// This is the default with the `expressive-defaults` feature disabled.
    #[cfg_attr(not(feature = "expressive-defaults"), default)]
    Standard,
}

pub fn fast_spatial(scheme: Scheme) -> Spring {
    match scheme {
        Scheme::Expressive => constants::FAST_SPATIAL_EXPRESSIVE,
        Scheme::Standard => constants::FAST_SPATIAL_STANDARD,
    }
}

pub fn fast_effects(scheme: Scheme) -> Spring {
    match scheme {
        Scheme::Expressive => constants::FAST_EFFECTS_EXPRESSIVE,
        Scheme::Standard => constants::FAST_EFFECTS_STANDARD,
    }
}

pub fn default_spatial(scheme: Scheme) -> Spring {
    match scheme {
        Scheme::Expressive => constants::DEFAULT_SPATIAL_EXPRESSIVE,
        Scheme::Standard => constants::DEFAULT_SPATIAL_STANDARD,
    }
}

pub fn default_effects(scheme: Scheme) -> Spring {
    match scheme {
        Scheme::Expressive => constants::DEFAULT_EFFECTS_EXPRESSIVE,
        Scheme::Standard => constants::DEFAULT_EFFECTS_STANDARD,
    }
}

pub fn slow_spatial(scheme: Scheme) -> Spring {
    match scheme {
        Scheme::Expressive => constants::SLOW_SPATIAL_EXPRESSIVE,
        Scheme::Standard => constants::SLOW_SPATIAL_STANDARD,
    }
}

pub fn slow_effects(scheme: Scheme) -> Spring {
    match scheme {
        Scheme::Expressive => constants::SLOW_EFFECTS_EXPRESSIVE,
        Scheme::Standard => constants::SLOW_EFFECTS_STANDARD,
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
pub struct SpringValue<T>
where
    T: Interpolable + Clone + PartialEq,
{
    pub from: T,
    pub to: T,
    pub spring: Spring,
    pub position: f32,
    pub velocity: f32,
    pub target: f32,
    pub time: Instant,
}

impl<T> SpringValue<T>
where
    T: Interpolable + Clone + PartialEq,
{
    pub const fn new(from: T, to: T, spring: Spring, now: Instant) -> Self {
        Self {
            from,
            to,
            spring,
            position: 0.0,
            velocity: 0.0,
            target: 0.0,
            time: now,
        }
    }

    pub fn value(&self) -> T {
        self.from
            .clone()
            .interpolate(self.to.clone(), self.position)
    }

    pub fn set_target(&mut self, target: T, now: Instant) {
        if self.to == target {
            return;
        }

        self.from = self.value();
        self.to = target;

        self.position = 0.0;
        self.target = 1.0;
        self.velocity = -self.velocity;
        self.time = now;
    }

    pub fn step(&mut self, now: Instant) {
        let delta_time = now.duration_since(self.time).as_secs_f32();
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

            let envelope = (decay * delta_time).exp();
            let angle = frequency * delta_time;
            let sin = angle.sin();
            let cos = angle.cos();

            x = envelope * (a * cos + b * sin);
            v = envelope * ((decay * a + frequency * b) * cos + (decay * b - frequency * a) * sin);
        } else if zeta == 1.0 {
            // Critically damped: fastest motion without overshoot.
            let envelope = (-omega * delta_time).exp();
            let b = v0 + omega * x0;

            x = envelope * (x0 + b * delta_time);
            v = envelope * (v0 - omega * b * delta_time);
        } else {
            // Overdamped: slower motion without overshoot.
            let root = (zeta * zeta - 1.0).sqrt();
            let r1 = -omega * (zeta - root);
            let r2 = -omega * (zeta + root);

            let a = (v0 - r2 * x0) / (r1 - r2);
            let b = x0 - a;

            let e1 = (r1 * delta_time).exp();
            let e2 = (r2 * delta_time).exp();

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
