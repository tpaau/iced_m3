mod scrollable;
mod shadow;

pub use scrollable::*;
pub use shadow::*;

use iced::Color;

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

/// Returns the arithmetic average of the two input colors.
pub fn mix_colors(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r * (1.0 - t) + b.r * t,
        g: a.g * (1.0 - t) + b.g * t,
        b: a.b * (1.0 - t) + b.b * t,
        a: a.a * (1.0 - t) + b.a * t,
    }
}
