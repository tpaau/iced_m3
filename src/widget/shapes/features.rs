use crate::widget::shapes::{DISTANCE_EPSILON, cubic::Cubic};

pub enum FeatureError {
    NoCubics,
    NotContinous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureKind {
    Edge,
    ConvexCorner,
    ConcaveCorner,
}

/// While a polygon's shape can be drawn solely using a list of [`Cubic`] objects representing its raw
/// curves and lines, features add an extra layer of context to groups of cubics. Features group
/// cubics into (straight) edges, convex corners, or concave corners. For example, rounding a
/// rectangle adds many cubics around its edges, but the rectangle's overall number of corners
/// remains the same. [`Morph`] therefore uses this grouping for several reasons:
/// - Noise Reduction: Grouping cubics reduces the amount of noise introduced by individual cubics
///   (as seen in the rounded rectangle example).
/// - Mapping Base: The grouping serves as the base set for [`Morph`]'s mapping process.
/// - Curve Type Mapping: [`Morph`] maps similar curve types (convex, concave) together. Note that
///   edges or features created with [`ignorable`] are ignored in the default mapping.
///
/// By using features, you can manipulate polygon shapes with more context and control.
#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    pub kind: FeatureKind,
    pub is_ignorable: bool,
    pub cubics: Vec<Cubic>,
}

impl Feature {
    fn is_continuous(&self) -> bool {
        if let Some(mut prev_cubic) = self.cubics.first() {
            for cubic in &self.cubics[1..] {
                if (cubic.anchor_0.x - prev_cubic.anchor_1.x).abs() > DISTANCE_EPSILON
                    || (cubic.anchor_0.y - prev_cubic.anchor_1.y).abs() > DISTANCE_EPSILON
                {
                    return false;
                }
                prev_cubic = cubic
            }
            true
        } else {
            true
        }
    }

    fn check(&self) -> Result<(), FeatureError> {
        if self.cubics.is_empty() {
            return Err(FeatureError::NoCubics);
        }

        if !self.is_continuous() {
            return Err(FeatureError::NotContinous);
        }

        Ok(())
    }

    /// Group a list of [`Cubic`] objects to a feature that should be ignored in the default
    /// [`Morph`] mapping. The feature can have any indentation.
    ///
    /// Sometimes, it's helpful to ignore certain features when morphing shapes. This is because
    /// only the features you mark as important will be smoothly transitioned between the start
    /// and end shapes. Additionally, the default morph algorithm will try to match convex
    /// corners to convex corners and concave to concave. Marking features as ignorable will
    /// influence this matching. For example, given a 12-pointed star, marking all concave
    /// corners as ignorable will create a [`Morph`] that only considers the outer corners of the
    /// star. As a result, depending on the morphed to shape, the animation may have fewer
    /// intersections and rotations. Another example for the other way around is a [`Morph`]
    /// between a pointed up triangle to a square. Marking the square's top edge as a convex
    /// corner matches it to the triangle's upper corner. Instead of moving triangle's upper
    /// corner to one of rectangle's corners, the animation now splits the triangle to match
    /// squares' outer corners.
    ///
    /// Fails for lists of empty cubics or non-continuous cubics.
    pub fn ignorable(cubics: Vec<Cubic>) -> Result<Self, FeatureError> {
        let feature = Self {
            kind: FeatureKind::Edge,
            is_ignorable: true,
            cubics,
        };

        feature.check()?;

        Ok(feature)
    }

    /// Group a [`Cubic`] object to an edge (neither inward or outward identification in a shape).
    pub fn edge(cubic: Cubic) -> Self {
        Self {
            kind: FeatureKind::Edge,
            is_ignorable: false,
            cubics: vec![cubic],
        }
    }

    /// Group a list of [`Cubic`] objects to a convex corner (outward indentation in a shape).
    ///
    /// Fails for lists of empty cubics or non-continuous cubics.
    pub fn convex_corner(cubics: Vec<Cubic>) -> Result<Self, FeatureError> {
        let feature = Self {
            kind: FeatureKind::ConvexCorner,
            is_ignorable: false,
            cubics,
        };

        feature.check()?;

        Ok(feature)
    }

    /// Group a list of [`Cubic`] objects to a concave corner (inward indentation in a shape).
    ///
    /// Fails for lists of empty cubics or non-continuous cubics.
    pub fn concave_corner(cubics: Vec<Cubic>) -> Result<Self, FeatureError> {
        let feature = Self {
            kind: FeatureKind::ConcaveCorner,
            is_ignorable: false,
            cubics,
        };

        feature.check()?;

        Ok(feature)
    }
}
