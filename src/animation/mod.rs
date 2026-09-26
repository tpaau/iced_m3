pub mod motion;

mod constants {
    use std::time::Duration;

    pub const EMPHASIZED_ACCELERATE: (f32, f32, f32, f32) = (0.3, 0.0, 0.8, 0.15);
    pub const EMPHASIZED_DECELERATE: (f32, f32, f32, f32) = (0.05, 0.7, 0.1, 1.0);
    pub const STANDARD: (f32, f32, f32, f32) = (0.2, 0.0, 0.0, 1.0);
    pub const STANDARD_ACCELERATE: (f32, f32, f32, f32) = (0.3, 0.0, 1.0, 1.0);
    pub const STANDARD_DECELERATE: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);

    pub const SHORT_1: Duration = Duration::from_millis(50);
    pub const SHORT_2: Duration = Duration::from_millis(100);
    pub const SHORT_3: Duration = Duration::from_millis(150);
    pub const SHORT_4: Duration = Duration::from_millis(200);
    pub const MEDIUM_1: Duration = Duration::from_millis(250);
    pub const MEDIUM_2: Duration = Duration::from_millis(300);
    pub const MEDIUM_3: Duration = Duration::from_millis(350);
    pub const MEDIUM_4: Duration = Duration::from_millis(400);
    pub const LONG_1: Duration = Duration::from_millis(450);
    pub const LONG_2: Duration = Duration::from_millis(500);
    pub const LONG_3: Duration = Duration::from_millis(550);
    pub const LONG_4: Duration = Duration::from_millis(600);
    pub const EXTRA_LONG_1: Duration = Duration::from_millis(700);
    pub const EXTRA_LONG_2: Duration = Duration::from_millis(800);
    pub const EXTRA_LONG_3: Duration = Duration::from_millis(900);
    pub const EXTRA_LONG_4: Duration = Duration::from_millis(1000);
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;
use iced::{Color, Radians};

use crate::style::mix_colors;

fn cubic(t: f32, p1: f32, p2: f32) -> f32 {
    3.0 * (1.0 - t).powi(2) * t * p1 + 3.0 * (1.0 - t) * t.powi(2) * p2 + t.powi(3)
}

fn cubic_derivative(t: f32, p1: f32, p2: f32) -> f32 {
    3.0 * (1.0 - t).powi(2) * p1 + 6.0 * (1.0 - t) * t * (p2 - p1) + 3.0 * t.powi(2) * (1.0 - p2)
}

fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);

    // Solve x(u) = t using Newton-Raphson iteration.
    let mut u = t;

    for _ in 0..5 {
        let x = cubic(u, x1, x2) - t;
        let derivative = cubic_derivative(u, x1, x2);

        if derivative.abs() < f32::EPSILON {
            break;
        }

        u = (u - x / derivative).clamp(0.0, 1.0);
    }

    cubic(u, y1, y2)
}

fn cubic_bezier_tuple(t: f32, tuple: (f32, f32, f32, f32)) -> f32 {
    cubic_bezier(t, tuple.0, tuple.1, tuple.2, tuple.3)
}

fn cubic_component(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    let one_minus_t = 1.0 - t;

    one_minus_t.powi(3) * p0
        + 3.0 * one_minus_t.powi(2) * t * p1
        + 3.0 * one_minus_t * t.powi(2) * p2
        + t.powi(3) * p3
}

fn cubic_component_derivative(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    let one_minus_t = 1.0 - t;

    3.0 * one_minus_t.powi(2) * (p1 - p0)
        + 6.0 * one_minus_t * t * (p2 - p1)
        + 3.0 * t.powi(2) * (p3 - p2)
}

fn cubic_bezier_segment(
    x: f32,
    x0: f32,
    x1: f32,
    x2: f32,
    x3: f32,
    y0: f32,
    y1: f32,
    y2: f32,
    y3: f32,
) -> f32 {
    let mut u = ((x - x0) / (x3 - x0)).clamp(0.0, 1.0);

    for _ in 0..8 {
        let current_x = cubic_component(u, x0, x1, x2, x3);
        let error = current_x - x;
        let derivative = cubic_component_derivative(u, x0, x1, x2, x3);

        if derivative.abs() <= f32::EPSILON {
            break;
        }

        u = (u - error / derivative).clamp(0.0, 1.0);
    }

    cubic_component(u, y0, y1, y2, y3)
}

pub fn emphasized(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);

    if t <= 0.166666 {
        cubic_bezier_segment(t, 0.0, 0.05, 0.133333, 0.166666, 0.0, 0.0, 0.06, 0.4)
    } else {
        cubic_bezier_segment(t, 0.166666, 0.208333, 0.25, 1.0, 0.4, 0.82, 1.0, 1.0)
    }
}

pub(crate) fn midpoint_distance(x: f32) -> f32 {
    (2.0 * (x - 0.5).abs()).clamp(0.0, 1.0)
}

pub fn emphasized_accelerate(t: f32) -> f32 {
    cubic_bezier_tuple(t, constants::EMPHASIZED_ACCELERATE)
}

pub fn emphasized_decelerate(t: f32) -> f32 {
    cubic_bezier_tuple(t, constants::EMPHASIZED_DECELERATE)
}

pub fn standard(t: f32) -> f32 {
    cubic_bezier_tuple(t, constants::STANDARD)
}

pub fn standard_accelerate(t: f32) -> f32 {
    cubic_bezier_tuple(t, constants::STANDARD_ACCELERATE)
}

pub fn standard_decelerate(t: f32) -> f32 {
    cubic_bezier_tuple(t, constants::STANDARD_DECELERATE)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Iced(iced::animation::Easing),
    Emphasized,
    EmphasizedAccelerate,
    EmphasizedDecelerate,
    Standard,
    StandardAccelerate,
    StandardDecelerate,
}

impl Easing {
    pub fn value(self, t: f32) -> f32 {
        match self {
            Self::Iced(easing) => easing.value(t),
            Self::Emphasized => emphasized(t),
            Self::EmphasizedAccelerate => emphasized_accelerate(t),
            Self::EmphasizedDecelerate => emphasized_decelerate(t),
            Self::Standard => standard(t),
            Self::StandardAccelerate => standard_accelerate(t),
            Self::StandardDecelerate => standard_decelerate(t),
        }
    }
}

impl From<Easing> for iced::animation::Easing {
    fn from(easing: Easing) -> Self {
        match easing {
            Easing::Iced(easing) => easing,
            Easing::Emphasized => iced::animation::Easing::Custom(emphasized),
            Easing::EmphasizedAccelerate => iced::animation::Easing::Custom(emphasized_accelerate),
            Easing::EmphasizedDecelerate => iced::animation::Easing::Custom(emphasized_decelerate),
            Easing::Standard => iced::animation::Easing::Custom(standard),
            Easing::StandardAccelerate => iced::animation::Easing::Custom(standard_accelerate),
            Easing::StandardDecelerate => iced::animation::Easing::Custom(standard_decelerate),
        }
    }
}

pub trait Interpolable {
    fn interpolate(self, other: Self, t: f32) -> Self;
}

impl Interpolable for Color {
    fn interpolate(self, other: Self, t: f32) -> Self {
        mix_colors(self, other, t)
    }
}

impl Interpolable for f32 {
    fn interpolate(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        self * (1.0 - t) + other * t
    }
}

impl Interpolable for f64 {
    fn interpolate(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0) as f64;
        self * (1.0 - t) + other * t
    }
}

impl Interpolable for Radians {
    fn interpolate(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Radians(self.0 * (1.0 - t) + other.0 * t)
    }
}
