pub use lilt::*;

mod constants {
    pub const EMPHASIZED_ACCELERATE: (f32, f32, f32, f32) = (0.3, 0.0, 0.8, 0.15);
    pub const EMPHASIZED_DECELERATE: (f32, f32, f32, f32) = (0.05, 0.7, 0.1, 1.0);
    pub const STANDARD: (f32, f32, f32, f32) = (0.2, 0.0, 0.0, 1.0);
    pub const STANDARD_ACCELERATE: (f32, f32, f32, f32) = (0.3, 0.0, 1.0, 1.0);
    pub const STANDARD_DECELERATE: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

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
    Lilt(lilt::Easing),
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
            Self::Lilt(easing) => easing.value(t),
            Self::Emphasized => emphasized(t),
            Self::EmphasizedAccelerate => emphasized_accelerate(t),
            Self::EmphasizedDecelerate => emphasized_decelerate(t),
            Self::Standard => standard(t),
            Self::StandardAccelerate => standard_accelerate(t),
            Self::StandardDecelerate => standard_decelerate(t),
        }
    }
}

impl From<Easing> for lilt::Easing {
    fn from(easing: Easing) -> Self {
        match easing {
            Easing::Lilt(easing) => easing,
            Easing::Emphasized => lilt::Easing::Custom(emphasized),
            Easing::EmphasizedAccelerate => lilt::Easing::Custom(emphasized_accelerate),
            Easing::EmphasizedDecelerate => lilt::Easing::Custom(emphasized_decelerate),
            Easing::Standard => lilt::Easing::Custom(standard),
            Easing::StandardAccelerate => lilt::Easing::Custom(standard_accelerate),
            Easing::StandardDecelerate => lilt::Easing::Custom(standard_decelerate),
        }
    }
}
