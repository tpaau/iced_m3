use iced::{Point, Rectangle, Size, Vector};

use crate::{
    animation::Interpolable,
    widget::shapes::{
        DISTANCE_EPSILON,
        utils::{distance, dot_product, normalized_vector, rotate_vector_90},
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cubic {
    pub anchor_0: Point,
    pub control_0: Point,
    pub control_1: Point,
    pub anchor_1: Point,
}

fn interpolate_point(from: Point, to: Point, t: f32) -> Point {
    Point {
        x: from.x.interpolate(to.x, t),
        y: from.y.interpolate(to.y, t),
    }
}

impl Interpolable for Cubic {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            anchor_0: interpolate_point(self.anchor_0, other.anchor_0, t),
            control_0: interpolate_point(self.control_0, other.control_0, t),
            control_1: interpolate_point(self.control_1, other.control_1, t),
            anchor_1: interpolate_point(self.anchor_1, other.anchor_1, t),
        }
    }
}

impl Cubic {
    /// Generates an empty [`Cubic`] defined at `point`.
    pub fn empty(point: Point) -> Self {
        Self {
            anchor_0: point,
            control_0: point,
            control_1: point,
            anchor_1: point,
        }
    }

    /// Generates a bezier curve that is a straight line between the given anchor points. The
    /// control points lie 1/3 of the distance from their respective anchor points.
    pub fn straight_line(start: Point, end: Point) -> Self {
        Self {
            anchor_0: start,
            control_0: interpolate_point(start, end, 1.0 / 3.0),
            control_1: interpolate_point(start, end, 2.0 / 3.0),
            anchor_1: end,
        }
    }

    /// Generates a bezier curve that approximates a circular arc. The curve generated is the
    /// smallest of the two possible arcs around the entire 360-degree circle. Arcs of greater than
    /// 180 degrees should use more than one arc together. Note that `start` and `end` should be
    /// equidistant from the center.
    pub fn circular_arc(center: Point, start: Point, end: Point) -> Self {
        let start_d = normalized_vector(start.x - center.x, start.y - center.y);
        let end_d = normalized_vector(end.x - center.x, end.y - center.y);
        let start_rotated = rotate_vector_90(start_d);
        let end_rotated = rotate_vector_90(end_d);
        let clockwise = dot_product(
            start_rotated,
            Vector::new(end.x - center.x, end.y - center.y),
        ) >= 0.0;
        let cosa = dot_product(start_d, end_d);

        // start ~= end
        if cosa > 0.999 {
            return Self::straight_line(start, end);
        }

        let clockwise = if clockwise { 1.0 } else { -1.0 };
        let k = distance(Vector::new(start.x - center.x, start.y - center.y)) * 4.0 / 3.0
            * ((2.0 * (1.0 - cosa)).sqrt() - (1.0 - cosa.powi(2)).sqrt())
            / (1.0 - cosa)
            * clockwise;

        Self {
            anchor_0: start,
            control_0: Point::new(start.x + start_rotated.x * k, start.y + start_rotated.y * k),
            control_1: Point::new(end.x - end_rotated.x * k, end.y - end_rotated.y * k),
            anchor_1: end,
        }
    }

    /// Returns a point on the curve for parameter `t`, representing the proportional distance along
    /// the curve between its starting point at `anchor_0` and ending point at `anchor_1`.
    ///
    /// The distance along the curve between the anchor points, where 0 is at `anchor_0` and 1
    /// is at `anchor_1`.
    pub fn point_on_curve(&self, t: f32) -> Point {
        let u = 1.0 - t;
        Point::new(
            self.anchor_0.x * (u * u * u)
                + self.control_0.x * (3.0 * t * u * u)
                + self.control_1.x * (3.0 * t * t * u)
                + self.anchor_1.x * (t * t * t),
            self.anchor_0.y * (u * u * u)
                + self.control_0.y * (3.0 * t * u * u)
                + self.control_1.y * (3.0 * t * t * u)
                + self.anchor_1.y * (t * t * t),
        )
    }

    /// Returns two Cubics, created by splitting this curve at the given distance of `t` between the
    /// original starting and ending anchor points.
    pub fn split(&self, t: f32) -> (Self, Self) {
        let u = 1.0 - t;
        let point_on_curve = self.point_on_curve(t);

        (
            Self {
                anchor_0: self.anchor_0,
                control_0: Point::new(
                    self.anchor_0.x * u + self.control_0.x * t,
                    self.anchor_0.y * u + self.control_0.y * t,
                ),
                control_1: Point::new(
                    self.anchor_0.x * u.powi(2)
                        + self.control_0.x * (2.0 * u * t)
                        + self.control_1.x * t.powi(2),
                    self.anchor_0.y * u.powi(2)
                        + self.control_0.y * (2.0 * u * t)
                        + self.control_1.y * t.powi(2),
                ),
                anchor_1: point_on_curve,
            },
            Self {
                anchor_0: point_on_curve,
                control_0: Point::new(
                    self.control_0.x * u.powi(2)
                        + self.control_1.x * (2.0 * u * t)
                        + self.anchor_1.x * t.powi(2),
                    self.control_0.y * u.powi(2)
                        + self.control_1.y * (2.0 * u * t)
                        + self.anchor_1.y * t.powi(2),
                ),
                control_1: Point::new(
                    self.control_1.x * u + self.anchor_1.x * t,
                    self.control_1.y * u + self.anchor_1.y * t,
                ),
                anchor_1: self.anchor_1,
            },
        )
    }

    /// Utility function to reverse the control/anchor points for this curve.
    pub fn reverse(self) -> Self {
        Self {
            anchor_0: self.anchor_1,
            control_0: self.control_1,
            control_1: self.control_0,
            anchor_1: self.anchor_0,
        }
    }

    fn is_zero_length(&self) -> bool {
        (self.anchor_0.x - self.anchor_1.x).abs() < DISTANCE_EPSILON
            && (self.anchor_0.y - self.anchor_1.y).abs() < DISTANCE_EPSILON
    }

    pub fn calculate_bounds(&self, approximate: bool) -> Rectangle {
        if self.is_zero_length() {
            return Rectangle::new(self.anchor_0, Size::new(0.0, 0.0));
        }

        let mut min_x = self.anchor_0.x.min(self.anchor_1.x);
        let mut min_y = self.anchor_0.y.min(self.anchor_1.y);
        let mut max_x = self.anchor_0.x.max(self.anchor_1.x);
        let mut max_y = self.anchor_0.y.max(self.anchor_1.y);

        if approximate {
            // Approximate bounds use the bounding box of all anchors and controls
            let left = min_x.min(self.control_0.x.min(self.control_1.x));
            let top = min_y.min(self.control_0.y.min(self.control_1.y));
            let right = max_x.max(self.control_0.x.max(self.control_1.x));
            let bottom = max_y.max(self.control_0.y.max(self.control_1.y));

            return Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
        }

        // Find the derivative, which is a quadratic Bezier. Then we can solve for t using
        // the quadratic formula
        let xa =
            -self.anchor_0.x + 3.0 * self.control_0.x - 3.0 * self.control_1.x + self.anchor_1.x;
        let xb = 2.0 * self.anchor_0.x - 4.0 * self.control_0.x + 2.0 * self.control_1.x;
        let xc = -self.anchor_0.x + self.control_0.x;

        let is_zeroish = xa.abs() < DISTANCE_EPSILON;
        if is_zeroish {
            // Try Muller's method instead; it can find a single root when a is 0
            if xb != 0.0 {
                let t = 2.0 * xc / (-2.0 * xb);
                if (0.0..=1.0).contains(&t) {
                    let it = self.point_on_curve(t).x;
                    if it < min_x {
                        min_x = it
                    }
                    if it > max_x {
                        max_x = it
                    }
                }
            }
        } else {
            let xs = xb.powi(2) - 4.0 * xa * xc;
            if xs >= 0.0 {
                let t1 = (-xb + xs.sqrt()) / (2.0 * xa);
                if (0.0..=1.0).contains(&t1) {
                    let it = self.point_on_curve(t1).x;
                    if it < min_x {
                        min_x = it
                    }
                    if it > max_x {
                        max_x = it
                    }
                }

                let t2 = (-xb - xs.sqrt()) / (2.0 * xa);
                if (0.0..=1.0).contains(&t2) {
                    let it = self.point_on_curve(t2).x;
                    if it < min_x {
                        min_x = it
                    }
                    if it > max_x {
                        max_x = it
                    }
                }
            }
        }

        // Repeat the above for y coordinate
        let ya =
            -self.anchor_0.y + 3.0 * self.control_0.y - 3.0 * self.control_1.y + self.anchor_1.y;
        let yb = 2.0 * self.anchor_0.y - 4.0 * self.control_0.y + 2.0 * self.control_1.y;
        let yc = -self.anchor_0.y + self.control_0.y;

        let is_zeroish = ya.abs() < DISTANCE_EPSILON;
        if is_zeroish {
            if yb != 0.0 {
                let t = 2.0 * yc / (-2.0 * yb);
                if (0.0..=1.0).contains(&t) {
                    let it = self.point_on_curve(t).y;
                    if it < min_y {
                        min_y = it
                    }
                    if it > max_y {
                        max_y = it
                    }
                }
            }
        } else {
            let ys = yb.powi(2) - 4.0 * ya * yc;
            if ys >= 0.0 {
                let t1 = (-yb + ys.sqrt()) / (2.0 * ya);
                if (0.0..=1.0).contains(&t1) {
                    let it = self.point_on_curve(t1).y;
                    if it < min_y {
                        min_y = it
                    }
                    if it > max_y {
                        max_y = it
                    }
                }

                let t2 = (-yb - ys.sqrt()) / (2.0 * ya);
                if (0.0..=1.0).contains(&t2) {
                    let it = self.point_on_curve(t2).y;
                    if it < min_y {
                        min_y = it
                    }
                    if it > max_y {
                        max_y = it
                    }
                }
            }
        }

        let left = min_x;
        let top = min_y;
        let right = max_x;
        let bottom = max_y;

        Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top))
    }
}
