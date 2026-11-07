//! Polygon helpers used to clip the ripple to a rounded rectangle.
//!
//! iced 0.14 can only clip to rectangles and has no radial gradients, so the
//! ripple is built from polygons: a circle is clipped against the (convex)
//! rounded rectangle with the Sutherland-Hodgman algorithm.

use std::f32::consts::{FRAC_PI_2, PI};

use iced::{Point, Size, border::Radius};

use super::constant::CHORD_TOLERANCE;

fn cross(a: Point, b: Point, p: Point) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

fn segments_for(radius: f32, angle: f32) -> usize {
    if radius <= CHORD_TOLERANCE {
        return 1;
    }
    // sagitta = r * (1 - cos(step / 2)) <= tolerance
    let step = 2.0 * (1.0 - CHORD_TOLERANCE / radius).acos();
    ((angle / step).ceil() as usize).max(1)
}

/// Radii clamped so that they never overlap (CSS `border-radius` rules).
#[must_use]
pub fn clamp_radius(size: Size, radius: Radius) -> [f32; 4] {
    // Avoids overflow for "infinite" radii (`f32::MAX` is a common pill radius).
    let limit = size.width.max(size.height);
    let r = [
        radius.top_left.clamp(0.0, limit),
        radius.top_right.clamp(0.0, limit),
        radius.bottom_right.clamp(0.0, limit),
        radius.bottom_left.clamp(0.0, limit),
    ];
    let mut factor: f32 = 1.0;
    for (sum, side) in [
        (r[0] + r[1], size.width),
        (r[1] + r[2], size.height),
        (r[2] + r[3], size.width),
        (r[3] + r[0], size.height),
    ] {
        if sum > side && sum > 0.0 {
            factor = factor.min(side / sum);
        }
    }
    r.map(|value| value * factor)
}

/// Inscribed polygon of a rounded rectangle at the origin, clockwise on screen.
#[must_use]
pub fn rounded_rect_polygon(size: Size, radius: Radius) -> Vec<Point> {
    let [tl, tr, br, bl] = clamp_radius(size, radius);
    let (w, h) = (size.width, size.height);
    let corners = [
        (Point::new(tl, tl), tl, PI),
        (Point::new(w - tr, tr), tr, PI + FRAC_PI_2),
        (Point::new(w - br, h - br), br, 0.0),
        (Point::new(bl, h - bl), bl, FRAC_PI_2),
    ];

    let mut points = Vec::new();
    for (center, r, start) in corners {
        if r <= 0.0 {
            points.push(center);
            continue;
        }
        let n = segments_for(r, FRAC_PI_2);
        for i in 0..=n {
            let angle = start + FRAC_PI_2 * i as f32 / n as f32;
            points.push(Point::new(
                center.x + r * angle.cos(),
                center.y + r * angle.sin(),
            ));
        }
    }
    dedup(points)
}

/// Removes (nearly) coincident neighbours. Edges of zero length have no
/// direction, which would make the clipping test meaningless.
fn dedup(mut points: Vec<Point>) -> Vec<Point> {
    const EPSILON: f32 = 1e-3;
    let close = |a: Point, b: Point| (a.x - b.x).abs() < EPSILON && (a.y - b.y).abs() < EPSILON;
    points.dedup_by(|b, a| close(*a, *b));
    while points.len() > 1 && close(points[0], points[points.len() - 1]) {
        points.pop();
    }
    points
}

/// Polygon inscribed in the circle.
#[must_use]
pub fn circle_polygon(center: Point, radius: f32) -> Vec<Point> {
    let n = segments_for(radius, 2.0 * PI).max(8);
    (0..n)
        .map(|i| {
            let angle = 2.0 * PI * i as f32 / n as f32;
            Point::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        })
        .collect()
}

/// Clips `subject` against the convex polygon `clip`.
#[must_use]
pub fn clip_convex(subject: &[Point], clip: &[Point]) -> Vec<Point> {
    let area: f32 = clip
        .iter()
        .zip(clip.iter().cycle().skip(1))
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum();
    let sign = if area >= 0.0 { 1.0 } else { -1.0 };

    let mut output = subject.to_vec();
    for (a, b) in clip
        .iter()
        .copied()
        .zip(clip.iter().copied().cycle().skip(1))
    {
        if output.is_empty() {
            break;
        }
        if a == b {
            continue;
        }
        let input = std::mem::take(&mut output);
        let mut previous = *input.last().unwrap();
        let mut previous_d = sign * cross(a, b, previous);
        for current in input {
            let current_d = sign * cross(a, b, current);
            if current_d >= 0.0 {
                if previous_d < 0.0 {
                    output.push(intersect(previous, current, previous_d, current_d));
                }
                output.push(current);
            } else if previous_d >= 0.0 {
                output.push(intersect(previous, current, previous_d, current_d));
            }
            previous = current;
            previous_d = current_d;
        }
    }
    output
}

fn intersect(s: Point, e: Point, ds: f32, de: f32) -> Point {
    let t = ds / (ds - de);
    Point::new(s.x + (e.x - s.x) * t, s.y + (e.y - s.y) * t)
}

/// Whether `p` is inside the exact rounded rectangle (not the polygon),
/// allowing `tolerance` pixels of slack.
#[must_use]
pub fn inside_rounded_rect(p: Point, size: Size, radius: Radius, tolerance: f32) -> bool {
    if p.x < -tolerance
        || p.y < -tolerance
        || p.x > size.width + tolerance
        || p.y > size.height + tolerance
    {
        return false;
    }
    let [tl, tr, br, bl] = clamp_radius(size, radius);
    let corners = [
        (Point::new(tl, tl), tl, p.x < tl && p.y < tl),
        (
            Point::new(size.width - tr, tr),
            tr,
            p.x > size.width - tr && p.y < tr,
        ),
        (
            Point::new(size.width - br, size.height - br),
            br,
            p.x > size.width - br && p.y > size.height - br,
        ),
        (
            Point::new(bl, size.height - bl),
            bl,
            p.x < bl && p.y > size.height - bl,
        ),
    ];
    corners.into_iter().all(|(center, r, in_corner)| {
        !in_corner || (p.x - center.x).hypot(p.y - center.y) <= r + tolerance
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Small deterministic generator, so tests need no extra dependency.
    struct Lcg(u64);

    impl Lcg {
        fn next(&mut self) -> f32 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 40) as f32) / ((1u64 << 24) as f32)
        }
    }

    #[test]
    fn clipped_vertices_stay_inside_the_rounded_rect() {
        let mut rng = Lcg(42);
        for _ in 0..2000 {
            let size = Size::new(20.0 + rng.next() * 300.0, 20.0 + rng.next() * 200.0);
            let radius = Radius {
                top_left: rng.next() * 60.0,
                top_right: rng.next() * 60.0,
                bottom_right: rng.next() * 60.0,
                bottom_left: rng.next() * 60.0,
            };
            let center = Point::new(
                (rng.next() * 1.4 - 0.2) * size.width,
                (rng.next() * 1.4 - 0.2) * size.height,
            );
            let r = 1.0 + rng.next() * 400.0;

            let clip = rounded_rect_polygon(size, radius);
            for p in &clip {
                assert!(inside_rounded_rect(
                    *p,
                    size,
                    radius,
                    CHORD_TOLERANCE * 0.01
                ));
            }
            for p in clip_convex(&circle_polygon(center, r), &clip) {
                assert!(
                    inside_rounded_rect(p, size, radius, CHORD_TOLERANCE),
                    "{p:?} escaped {size:?} {radius:?}"
                );
            }
        }
    }

    #[test]
    fn polygon_deviation_is_within_tolerance() {
        let size = Size::new(100.0, 40.0);
        let radius = Radius::from(20.0);
        let polygon = rounded_rect_polygon(size, radius);
        // The midpoint of every chord is at most `CHORD_TOLERANCE` inside the arc.
        for (a, b) in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
            let mid = Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
            assert!(inside_rounded_rect(mid, size, radius, 1e-3));
        }
    }

    #[test]
    fn touching_corner_radii_do_not_create_empty_clips() {
        // The arcs meet exactly at the middle of the sides here.
        let size = Size::new(200.0, 80.0);
        let clip = rounded_rect_polygon(size, Radius::from(40.0));
        let out = clip_convex(&circle_polygon(Point::new(78.5, 34.6), 75.0), &clip);
        assert!(out.len() >= 3);
    }

    #[test]
    fn pill_radius_is_clamped() {
        let size = Size::new(40.0, 40.0);
        let r = clamp_radius(size, Radius::from(f32::MAX));
        assert!(r.iter().all(|r| (r - 20.0).abs() < 1e-4), "{r:?}");
    }

    #[test]
    fn clipping_a_big_circle_returns_the_rect() {
        let size = Size::new(100.0, 40.0);
        let clip = rounded_rect_polygon(size, Radius::from(0.0));
        let out = clip_convex(&circle_polygon(Point::new(50.0, 20.0), 1000.0), &clip);
        assert_eq!(out.len(), 4);
    }

    #[test]
    fn tessellation_is_cheap() {
        let size = Size::new(300.0, 200.0);
        let radius = Radius::from(28.0);
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let clip = rounded_rect_polygon(size, radius);
            for k in 0..=super::super::constant::GRADIENT_RINGS {
                let r = 200.0 + 100.0 * k as f32 / 31.0;
                std::hint::black_box(clip_convex(
                    &circle_polygon(Point::new(150.0, 100.0), r),
                    &clip,
                ));
            }
        }
        let per_frame = start.elapsed() / 100;
        eprintln!("polygon generation per frame: {per_frame:?}");
        assert!(per_frame.as_millis() < 20, "{per_frame:?}");
    }
}
