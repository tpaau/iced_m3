mod scrollable;
mod shadow;

pub use scrollable::*;
pub use shadow::*;

use iced::Color;
use palette::{IntoColor, LinSrgb, Oklab, Srgb};

pub const DIM_ALPHA: f32 = 0.7;

pub const DRAGGED_STATE_LAYER_OPACITY: f32 = 0.16;
pub const PRESSED_STATE_LAYER_OPACITY: f32 = 0.1;
pub const FOCUS_STATE_LAYER_OPACITY: f32 = 0.1;
pub const HOVER_STATE_LAYER_OPACITY: f32 = 0.08;
pub const DISABLED_STATE_LAYER_OPACITY: f32 = 0.38;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateLayer {
    pub idle: Color,
    pub hovered: Color,
    pub focused: Color,
    pub pressed: Color,
    pub dragged: Color,
}

impl StateLayer {
    pub fn new(color: Color) -> Self {
        Self {
            idle: Color::TRANSPARENT,
            hovered: color.scale_alpha(HOVER_STATE_LAYER_OPACITY),
            focused: color.scale_alpha(FOCUS_STATE_LAYER_OPACITY),
            pressed: color.scale_alpha(PRESSED_STATE_LAYER_OPACITY),
            dragged: color.scale_alpha(DRAGGED_STATE_LAYER_OPACITY),
        }
    }
}

// TODO: Remove this after Oklab makes it into stable
pub fn mix_colors(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    let a_alpha = a.a;
    let b_alpha = b.a;

    let a_lab: Oklab = Srgb::new(a.r, a.g, a.b).into_linear().into_color();

    let b_lab: Oklab = Srgb::new(b.r, b.g, b.b).into_linear().into_color();

    let mixed_lab = Oklab::new(
        a_lab.l + (b_lab.l - a_lab.l) * t,
        a_lab.a + (b_lab.a - a_lab.a) * t,
        a_lab.b + (b_lab.b - a_lab.b) * t,
    );

    let linear_rgb: LinSrgb<f32> = mixed_lab.into_color();
    let rgb: Srgb<f32> = Srgb::from_linear(linear_rgb);

    Color {
        r: rgb.red.clamp(0.0, 1.0),
        g: rgb.green.clamp(0.0, 1.0),
        b: rgb.blue.clamp(0.0, 1.0),
        a: a_alpha + (b_alpha - a_alpha) * t,
    }
}

/// Returns the arithmetic average of the two input colors.
pub fn mix_colors_srgb(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r * (1.0 - t) + b.r * t,
        g: a.g * (1.0 - t) + b.g * t,
        b: a.b * (1.0 - t) + b.b * t,
        a: a.a * (1.0 - t) + b.a * t,
    }
}
