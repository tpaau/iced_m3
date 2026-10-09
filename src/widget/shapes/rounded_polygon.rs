use iced::{Point, Rectangle, Size};

use crate::widget::shapes::{cubic::Cubic, utils::distance_squared};

#[derive(Debug, Clone, PartialEq)]
pub struct RoundedPolygon {
    pub center: Point,
    pub cubics: Vec<Cubic>,
}

impl RoundedPolygon {
    pub fn new(center: Point, cubics: Vec<Cubic>) -> Self {
        Self { center, cubics }
    }

    /// Like [`calculate_bounds`], this function calculates the axis-aligned bounds of the object
    /// and returns that rectangle. But this function determines the max dimension of the shape (by
    /// calculating the distance from its center to the start and midpoint of each curve) and
    /// returns a square which can be used to hold the object in any rotation. This function can be
    /// used, for example, to calculate the max size of a UI element meant to hold this shape in any
    /// rotation.
    pub fn max_bounds(&self) -> Rectangle {
        let mut max_dist_squared: f32 = 0.0;
        for cubic in &self.cubics {
            let anchor_distance = distance_squared(
                cubic.anchor_0.x - self.center.x,
                cubic.anchor_0.y - self.center.y,
            );
            let middle_point = cubic.point_on_curve(0.5);
            let middle_distance = distance_squared(
                middle_point.x - self.center.x,
                middle_point.y - self.center.y,
            );
            max_dist_squared = max_dist_squared.max(anchor_distance.max(middle_distance));
        }
        let distance = max_dist_squared.sqrt();

        let left = self.center.x - distance;
        let top = self.center.y - distance;
        let right = self.center.x + distance;
        let bottom = self.center.y + distance;

        Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top))
    }
}
