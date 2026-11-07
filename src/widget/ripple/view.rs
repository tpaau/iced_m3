use std::time::Instant;

use iced::Point;

use super::{constant::*, model::Model, update::Context};

/// A soft-edged disc. Coordinates are relative to the top-left of the bounds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Disc {
    pub center: Point,
    pub radius: f32,
    /// Fraction of `radius` that is fully opaque.
    pub solid_ratio: f32,
    /// Multiplier for the alpha of the pressed state layer color.
    pub intensity: f32,
}

/// Pure description of what has to be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Scene {
    /// Multiplier for the alpha of the hovered state layer color.
    pub hover: Option<f32>,
    pub press: Option<Disc>,
}

#[must_use]
pub fn view(model: &Model, context: Context, now: Instant) -> Scene {
    if !context.enabled {
        return Scene::default();
    }

    let hover = model.hover.value(now);
    let hover = (hover > 0.0).then_some(hover);

    let intensity = model.fade.value(now);
    let press = model.press.and_then(|press| {
        // The official ripple has `scale == Infinity` here and shows nothing.
        if intensity <= 0.0 || press.initial_size <= 0.0 || !press.scale.is_finite() {
            return None;
        }

        let e = press.grow(now);
        let center = Point::new(
            press.start.x + (press.end.x - press.start.x) * e,
            press.start.y + (press.end.y - press.start.y) * e,
        );
        let initial_radius = press.initial_size / 2.0;
        let radius = initial_radius * (1.0 + (press.scale - 1.0) * e);
        let solid_ratio =
            ((initial_radius - GRADIENT_FEATHER_PX) / initial_radius).max(GRADIENT_SOLID_MIN_RATIO);

        Some(Disc {
            center,
            radius,
            solid_ratio,
            intensity,
        })
    });

    Scene { hover, press }
}
