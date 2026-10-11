use iced::{Point, Rectangle, Size, Vector};

use crate::widget::shapes::{cubic::Cubic, features::Feature, utils::distance_squared};

#[derive(Debug, Clone, PartialEq)]
pub struct RoundedPolygon {
    pub center: Point,
    pub cubics: Vec<Cubic>,
    pub features: Vec<Feature>,
}

impl RoundedPolygon {
    pub fn new(center: Point, cubics: Vec<Cubic>, features: Vec<Feature>) -> Self {
        Self {
            center,
            cubics,
            features,
        }
    }

    /// Calculates the axis-aligned bounds of the object.
    pub fn bounds(&self, approximate: bool) -> Rectangle {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for cubic in &self.cubics {
            let bounds = cubic.calculate_bounds(approximate);
            min_x = min_x.min(bounds.x);
            min_y = min_y.min(bounds.y);
            max_x = max_x.max(bounds.x + bounds.width);
            max_y = max_y.max(bounds.y + bounds.height);
        }

        Rectangle::new(
            Point::new(min_x, min_y),
            Size::new(max_x - min_x, max_y - min_y),
        )
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
            let anchor_distance = distance_squared(Vector::new(
                cubic.anchor_0.x - self.center.x,
                cubic.anchor_0.y - self.center.y,
            ));
            let middle_point = cubic.point_on_curve(0.5);
            let middle_distance = distance_squared(Vector::new(
                middle_point.x - self.center.x,
                middle_point.y - self.center.y,
            ));
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
