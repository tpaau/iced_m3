use std::rc::Rc;

use iced::{Point, Rectangle, Size};

use crate::{
    animation::Interpolable,
    widget::shapes::{
        Error,
        cubic::Cubic,
        feature_mapping::feature_mapper,
        polygon_measure::{LengthMeasurer, MeasuredCubic, MeasuredPolygon},
        rounded_polygon::RoundedPolygon,
        utils::positive_modulo,
    },
};

pub(crate) const DISTANCE_EPSILON: f32 = 1e-4;
pub(crate) const ANGLE_EPSILON: f32 = 1e-6;

/// This class is used to animate between start and end polygons objects.
///
/// Morphing between arbitrary objects can be problematic because it can be difficult to determine
/// how the points of a given shape map to the points of some other shape. [`Morph`] simplifies the
/// problem by only operating on [`RoundedPolygon`] objects, which are known to have similar,
/// contiguous structures. For one thing, the shape of a polygon is contiguous from start to end
/// (compared to an arbitrary Path object, which could have one or more `move_to` operations in the
/// shape). Also, all edges of a polygon shape are represented by [`Cubic`] objects, thus the start
/// and end shapes use similar operations. Two Polygon shapes then only differ in the quantity and
/// placement of their curves. The morph works by determining how to map the curves of the two
/// shapes together (based on proximity and other information, such as distance to polygon vertices
/// and concavity), and splitting curves when the shapes do not have the same number of curves or
/// when the curve placement within the shapes is very different.
pub struct Morph {
    start: RoundedPolygon,
    end: RoundedPolygon,
    morph_match: Vec<(Cubic, Cubic)>,
}

impl Morph {
    /// Creates the structure used to animate between the start and end shapes. The technique is to
    /// match geometry (curves) between the shapes when and where possible, and to create
    /// new/placeholder curves when necessary (when one of the shapes has more curves than the
    /// other). The result is a list of pairs of Cubic curves. Those curves are the matched pairs:
    /// the first of each pair holds the geometry of the start shape, the second holds the geometry
    /// for the end shape. Changing the progress of a Morph object simply interpolates between all
    /// pairs of curves for the morph shape.
    ///
    /// Curves on both shapes are matched by running the [`Measurer`] to determine where the points
    /// are in each shape (proportionally, along the outline), and then running [`feature_mapper`]
    /// which decides how to map (match) all of the curves with each other.
    pub fn new(p1: RoundedPolygon, p2: RoundedPolygon) -> Result<Self, Error> {
        // Measure polygons, returns lists of measured cubics for each polygon, which
        // we then use to match start/end curves
        let length_measurer = Rc::new(LengthMeasurer::new());
        let measured_polygon1 =
            MeasuredPolygon::measure_polygon(length_measurer.clone(), p1.clone())?;
        let measured_polygon2 = MeasuredPolygon::measure_polygon(length_measurer, p2.clone())?;

        // features_start and end will contain the list of corners (just the inner circular curve)
        // along with the progress at the middle of those corners. These measurement values
        // are then used to compare and match between the two polygons
        let features1 = measured_polygon1.features.clone();
        let features2 = measured_polygon2.features.clone();

        // Map features: doubleMapper is the result of mapping the features in each shape to the
        // closest feature in the other shape.
        // Given a progress in one of the shapes it can be used to find the corresponding
        // progress in the other shape (in both directions)
        let double_mapper = feature_mapper(features1, features2)?;

        // cut point on poly2 is the mapping of the 0 point on poly1
        let polygon2_cut_point = double_mapper.map(0.0)?;

        // Cut and rotate.
        // Polygons start at progress 0, and the featureMapper has decided that we want to match
        // progress 0 in the first polygon to `polygon2CutPoint` on the second polygon.
        // So we need to cut the second polygon there and "rotate it", so as we walk through
        // both polygons we can find the matching.
        // The resulting bs1/2 are MeasuredPolygons, whose MeasuredCubics start from
        // outlineProgress=0 and increasing until outlineProgress=1
        let bs1 = measured_polygon1;
        let bs2 = measured_polygon2.cut_and_shift(polygon2_cut_point)?;

        // Match
        // Now we can compare the two lists of measured cubics and create a list of pairs
        // of cubics [ret], which are the start/end curves that represent the Morph object
        // and the start and end shapes, and which can be interpolated to animate the
        // between those shapes.
        let mut ret: Vec<(Cubic, Cubic)> = Vec::new();
        // i1/i2 are the indices of the current cubic on the start and end shapes
        let mut i1 = 0;
        let mut i2 = 0;
        // b1, b2 are the current measured cubic for each polygon
        let mut b1 = bs1.cubics.get(i1).cloned();
        i1 += 1;
        let mut b2 = bs2.cubics.get(i2).cloned();
        i2 += 1;

        // Iterate until all curves are accounted for and matched
        while let Some(b1_unwrapped) = b1.clone()
            && let Some(b2_unwrapped) = b2.clone()
        {
            // Progresses are in shape_start's perspective
            // b_start_a, b_end_a are ending progress values of current measured cubics in [0,1] range
            let b1a = if i1 == bs1.cubics.len() {
                1.0
            } else {
                b1_unwrapped.outline_progress().end()
            };
            let b2a = if i2 == bs2.cubics.len() {
                1.0
            } else {
                double_mapper.map_back(positive_modulo(
                    b2_unwrapped.outline_progress().end() + polygon2_cut_point,
                    1.0,
                ))?
            };
            let minb = b2a.min(b1a);

            // min b is the progress at which the curve that ends first ends.
            // If both curves ends roughly there, no cutting is needed, we have a match.
            // If one curve extends beyond, we need to cut it.
            let (seg1, newb1): (MeasuredCubic, Option<MeasuredCubic>) =
                if b1a > minb + ANGLE_EPSILON {
                    let val = b1_unwrapped.cut_at_progress(minb)?;
                    (val.0, Some(val.1))
                } else {
                    let next = (b1_unwrapped, bs1.cubics.get(i1).cloned());
                    i1 += 1;
                    next
                };
            let (seg2, newb2): (MeasuredCubic, Option<MeasuredCubic>) =
                if b2a > minb + ANGLE_EPSILON {
                    let val = b2_unwrapped.cut_at_progress(positive_modulo(
                        double_mapper.map(minb)? - polygon2_cut_point,
                        1.0,
                    ))?;
                    (val.0, Some(val.1))
                } else {
                    let next = (b2_unwrapped, bs2.cubics.get(i2).cloned());
                    i2 += 1;
                    next
                };
            ret.push((seg1.cubic, seg2.cubic));
            b1 = newb1;
            b2 = newb2;
        }
        assert!(
            b1.is_none() && b2.is_none(),
            "Expected both Polygon's Cubic to be fully matched"
        );

        Ok(Self {
            start: p1,
            end: p2,
            morph_match: ret,
        })
    }

    /// Calculates the axis-aligned bounds of the object.
    pub fn bounds(&self, approximate: bool) -> Rectangle {
        let bounds = self.start.bounds(approximate);
        let mut min_x = bounds.x;
        let mut min_y = bounds.y;
        let mut max_x = bounds.x + bounds.width;
        let mut max_y = bounds.y + bounds.height;

        let bounds = self.end.bounds(approximate);
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.width);
        max_y = max_y.max(bounds.y + bounds.height);

        Rectangle::new(
            Point::new(min_x, min_y),
            Size::new(max_x - min_x, max_y - min_y),
        )
    }

    /// Like [`bounds`], this function calculates the axis-aligned bounds of the object and returns
    /// that rectangle. But this function determines the max dimension of the shape (by calculating
    /// the distance from its center to the start and midpoint of each curve) and returns a square
    /// which can be used to hold the object in any rotation. This function can be used, for
    /// example, to calculate the max size of a UI element meant to hold this shape in any rotation.
    pub fn max_bounds(&self) -> Rectangle {
        let bounds = self.start.max_bounds();
        let mut min_x = bounds.x;
        let mut min_y = bounds.y;
        let mut max_x = bounds.x + bounds.width;
        let mut max_y = bounds.y + bounds.height;

        let bounds = self.end.max_bounds();
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.width);
        max_y = max_y.max(bounds.y + bounds.height);

        Rectangle::new(
            Point::new(min_x, min_y),
            Size::new(max_x - min_x, max_y - min_y),
        )
    }

    /// Returns a representation of the morph object at a given [`progress`] value as a list of
    /// [`Cubic`]s. Note that this function causes a new list to be created and populated, so there
    /// is some overhead.
    ///
    /// `progress` is a value from 0 to 1 that determines the morph's current shape, between the
    /// start and end shapes provided at construction time. A value of 0 results in the start
    /// shape, a value of 1 results in the end shape, and any value in between results in a shape
    /// which is a linear interpolation between those two shapes. The range is generally \[0..1\]
    /// and values outside could result in undefined shapes, but values close to (but outside) the
    /// range can be used to get an exaggerated effect (e.g., for a bounce or overshoot animation).
    pub fn cubics(&self, progress: f32) -> Vec<Cubic> {
        // The first/last mechanism here ensures that the final anchor point in the shape
        // exactly matches the first anchor point. There can be rendering artifacts introduced
        // by those points being slightly off, even by much less than a pixel
        let mut cubics = Vec::with_capacity(self.morph_match.len());
        let mut first_cubic: Option<Cubic> = None;
        let mut last_cubic: Option<Cubic> = None;
        for (cubic1, cubic2) in self.morph_match.clone() {
            let cubic = cubic1.interpolate(cubic2, progress);
            if first_cubic.is_none() {
                first_cubic = Some(cubic);
            }
            if let Some(last_cubic) = last_cubic {
                cubics.push(last_cubic);
            }
            last_cubic = Some(cubic)
        }

        if let Some(last_cubic) = last_cubic
            && let Some(first_cubic) = first_cubic
        {
            let cubic = Cubic {
                anchor_0: last_cubic.anchor_0,
                control_0: last_cubic.control_0,
                control_1: last_cubic.control_1,
                anchor_1: first_cubic.anchor_0,
            };
            cubics.push(cubic)
        }

        cubics
    }
}
