use iced::{
    Border, Color, Point, Rectangle,
    advanced::renderer,
    border::Radius,
    widget::canvas::{self, Fill, Frame, Path, fill::Rule},
};
use iced_widget::renderer::geometry;

use super::{
    clip::{circle_polygon, clip_convex, rounded_rect_polygon},
    constant::GRADIENT_RINGS,
    view::{Disc, Scene},
};
use crate::style::StateLayer;

/// Draws a [`Scene`] inside `bounds`, clipped to the rounded rectangle.
///
/// The radial gradient is approximated with [`GRADIENT_RINGS`] non-overlapping
/// rings, each being the area between two clipped circles (even-odd fill).
pub fn draw<R: geometry::Renderer>(
    renderer: &mut R,
    bounds: Rectangle,
    radius: Radius,
    layer: StateLayer,
    scene: Scene,
) {
    if let Some(hover) = scene.hover {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border::default().rounded(radius),
                ..Default::default()
            },
            layer.hovered.scale_alpha(hover),
        );
    }

    if let Some(disc) = scene.press {
        let color = layer.pressed.scale_alpha(disc.intensity);
        if color.a <= 0.0 || bounds.width <= 0.0 || bounds.height <= 0.0 {
            return;
        }

        let mut frame = Frame::new(renderer, bounds.size());
        for (path, alpha) in press_paths(bounds, radius, disc) {
            frame.fill(
                &path,
                Fill {
                    style: canvas::Style::Solid(Color {
                        a: color.a * alpha,
                        ..color
                    }),
                    rule: Rule::EvenOdd,
                },
            );
        }
        let geometry = frame.into_geometry();
        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

/// Paths with the alpha multiplier of each, from the opaque core outwards.
fn press_paths(bounds: Rectangle, radius: Radius, disc: Disc) -> Vec<(Path, f32)> {
    if disc.radius < 0.5 {
        return Vec::new();
    }
    let clip = rounded_rect_polygon(bounds.size(), radius);
    let solid = disc.radius * disc.solid_ratio;

    let polygon = |r: f32| clip_convex(&circle_polygon(disc.center, r), &clip);
    let radii = |k: usize| solid + (disc.radius - solid) * k as f32 / GRADIENT_RINGS as f32;

    let mut paths = Vec::with_capacity(GRADIENT_RINGS + 1);
    let mut inner = polygon(radii(0));
    if inner.len() >= 3 {
        paths.push((polygon_path(&[&inner]), 1.0));
    }

    for k in 0..GRADIENT_RINGS {
        let outer = polygon(radii(k + 1));
        if outer.len() >= 3 {
            let alpha = 1.0 - (k as f32 + 0.5) / GRADIENT_RINGS as f32;
            let rings: Vec<&[Point]> = if inner.len() >= 3 {
                vec![&outer, &inner]
            } else {
                vec![&outer]
            };
            paths.push((polygon_path(&rings), alpha));
        }
        inner = outer;
    }
    paths
}

fn polygon_path(polygons: &[&[Point]]) -> Path {
    Path::new(|builder| {
        for polygon in polygons {
            let mut points = polygon.iter();
            if let Some(first) = points.next() {
                builder.move_to(*first);
                for point in points {
                    builder.line_to(*point);
                }
                builder.close();
            }
        }
    })
}
