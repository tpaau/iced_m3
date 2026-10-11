use std::rc::Rc;

use crate::widget::shapes::{
    Error,
    cubic::Cubic,
    feature_mapping::ProgressableFeature,
    features::FeatureKind,
    morph::DISTANCE_EPSILON,
    rounded_polygon::RoundedPolygon,
    utils::{distance, positive_modulo},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ProgressRange {
    start: f32,
    end: f32,
}

impl ProgressRange {
    pub fn new(start: f32, end: f32) -> Result<Self, Error> {
        match end >= start {
            true => Ok(Self { start, end }),
            false => Err(Error::InvalidRange),
        }
    }

    pub fn start(&self) -> f32 {
        self.start
    }

    pub fn end(&self) -> f32 {
        self.end
    }
}

#[derive(Clone)]
pub struct MeasuredCubic {
    pub(crate) cubic: Cubic,
    measured_size: f32,
    measurer: Rc<dyn Measurer>,
    outline_progress: ProgressRange,
}

impl MeasuredCubic {
    pub fn new(measurer: Rc<dyn Measurer>, cubic: Cubic, outline_progress: ProgressRange) -> Self {
        let measured_size = measurer.measure_cubic(cubic);
        Self {
            cubic,
            measured_size,
            outline_progress,
            measurer,
        }
    }

    fn update_progress_range(&mut self, outline_progress: ProgressRange) {
        self.outline_progress = outline_progress;
    }

    /// Cut this MeasuredCubic into two MeasuredCubics at the given outline progress value.
    pub fn cut_at_progress(&self, progress: f32) -> Result<(Self, Self), Error> {
        let bounded_cut_outline_progress =
            progress.clamp(self.outline_progress.start, self.outline_progress.end);
        let outline_progress_size = self.outline_progress.end - self.outline_progress.start;
        let progress_from_start = bounded_cut_outline_progress - self.outline_progress.start;

        let relative_progress = progress_from_start / outline_progress_size;
        let t = self
            .measurer
            .find_cubic_cut_point(self.cubic, relative_progress * self.measured_size);
        assert!(
            (0.0..=1.0).contains(&t),
            "Cubic cut point is expected to be between 0 and 1"
        );

        let (c1, c2) = self.cubic.split(t);
        Ok((
            MeasuredCubic::new(self.measurer.clone(), c1, self.outline_progress),
            MeasuredCubic::new(self.measurer.clone(), c2, self.outline_progress),
        ))
    }

    pub fn outline_progress(&self) -> ProgressRange {
        self.outline_progress
    }
}

#[derive(Clone)]
pub struct MeasuredPolygon {
    measurer: Rc<dyn Measurer>,
    pub cubics: Vec<MeasuredCubic>,
    pub features: Vec<ProgressableFeature>,
}

impl MeasuredPolygon {
    pub fn new(
        measurer: Rc<dyn Measurer>,
        features: Vec<ProgressableFeature>,
        cubics: Vec<Cubic>,
        outline_progress: Vec<f32>,
    ) -> Result<Self, Error> {
        assert!(
            outline_progress.len() != cubics.len() + 1,
            "Outline progress size is expected to be the cubics size + 1"
        );
        assert!(
            *outline_progress.first().unwrap() == 0.0,
            "First outline progress value is expected to be zero"
        );
        assert!(
            *outline_progress.last().unwrap() == 1.0,
            "Last outline progress value is expected to be one"
        );

        let mut measured_cubics = Vec::new();
        let mut start_outline_progress = 0.0;
        for index in 0..cubics.len() {
            // Filter out "empty" cubics
            if (outline_progress[index + 1] - outline_progress[index]) > DISTANCE_EPSILON {
                measured_cubics.push(MeasuredCubic::new(
                    measurer.clone(),
                    cubics[index],
                    ProgressRange::new(start_outline_progress, outline_progress[index + 1])?,
                ));
                // The next measured cubic will start exactly where this one ends.
                start_outline_progress = outline_progress[index + 1];
            }
        }

        measured_cubics.last_mut().unwrap().outline_progress.end = 1.0;

        Ok(Self {
            measurer,
            cubics: measured_cubics,
            features,
        })
    }

    pub fn cut_and_shift(&self, cutting_point: f32) -> Result<MeasuredPolygon, Error> {
        assert!(
            (0.0..=1.0).contains(&cutting_point),
            "Cutting point is expected to be between 0 and 1"
        );

        let target_index = self
            .cubics
            .iter()
            .position(|cubic| {
                (cubic.outline_progress.start..=cubic.outline_progress.end).contains(&cutting_point)
            })
            .expect("No cubic is in the cutting point");
        let target = self.cubics[target_index].clone();

        let (b1, b2) = target
            .cut_at_progress(cutting_point)
            .expect("Couldn't cut the cubic");

        let mut ret_cubics = Vec::with_capacity(self.cubics.len() + 1);
        ret_cubics.push(b2.cubic);
        for i in 1..self.cubics.len() {
            ret_cubics.push(self.cubics[(i + target_index) % self.cubics.len()].cubic);
        }
        ret_cubics.push(b1.cubic);

        let mut ret_outline_progress = Vec::with_capacity(self.cubics.len() + 2);
        for index in 0..self.cubics.len() + 2 {
            let progress = match index {
                0 => 0.0,
                i if i == self.cubics.len() + 1 => 1.0,
                _ => {
                    let cubic_index = (target_index + index - 1) % self.cubics.len();
                    (self.cubics[cubic_index].outline_progress.end - cutting_point).rem_euclid(1.0)
                }
            };

            ret_outline_progress.push(progress);
        }

        let mut new_features = Vec::new();
        for feature in &self.features {
            new_features.push(ProgressableFeature {
                progress: positive_modulo(feature.progress - cutting_point, 1.0),
                feature: feature.feature.clone(),
            });
        }

        MeasuredPolygon::new(
            self.measurer.clone(),
            new_features,
            ret_cubics,
            ret_outline_progress,
        )
    }

    pub fn measure_polygon(
        measurer: Rc<dyn Measurer>,
        polygon: RoundedPolygon,
    ) -> Result<Self, Error> {
        let mut cubics = Vec::new();
        let mut feature_to_cubic = Vec::new();

        for feature in polygon.features {
            for index in 0..feature.cubics.len() {
                if matches!(
                    feature.kind,
                    FeatureKind::ConvexCorner | FeatureKind::ConcaveCorner
                ) && index == feature.cubics.len() / 2
                {
                    feature_to_cubic.push((feature.clone(), cubics.len()));
                }
                cubics.push(feature.cubics[index]);
            }
        }

        let mut measures = Vec::with_capacity(cubics.len() + 1);
        let mut measure = 0.0;
        measures.push(measure);

        for cubic in &cubics {
            let length = measurer.measure_cubic(*cubic);
            assert!(
                length >= 0.0,
                "Measured cubic is expected to be greater or equal to zero"
            );
            measure += length;
            measures.push(measure);
        }
        let total_measure = measures.last().unwrap().clone();

        let mut outline_progress = Vec::with_capacity(measures.len());
        for measure in measures {
            outline_progress.push(measure / total_measure);
        }

        // return MeasuredPolygon(measurer, features, cubics, outlineProgress)

        let mut features = Vec::with_capacity(feature_to_cubic.len());
        for i in 0..feature_to_cubic.len() {
            let ix = feature_to_cubic[i].1;
            features.push(ProgressableFeature {
                progress: positive_modulo(
                    (outline_progress[ix] + outline_progress[ix + 1]) / 2.0,
                    1.0,
                ),
                feature: feature_to_cubic[i].0.clone(),
            });
        }

        MeasuredPolygon::new(measurer, features, cubics, outline_progress)
    }
}

// Interface for measuring a cubic. Implementations can use whatever algorithm desired to produce
// these measurement values.
pub trait Measurer {
    /// Returns size of given cubic, according to however the implementation wants to measure the
    /// size (angle, length, etc). It has to be greater or equal to 0.
    fn measure_cubic(&self, cubic: Cubic) -> f32;

    /// Given a cubic and a measure that should be between 0 and the value returned by measureCubic
    /// (If not, it will be capped), finds the parameter t of the cubic at which that measure is
    /// reached.
    fn find_cubic_cut_point(&self, cubic: Cubic, m: f32) -> f32;
}

/// Approximates the arc lengths of cubics by splitting the arc into segments and calculating their
/// sizes. The more segments, the more accurate the result will be to the true arc length. The
/// default implementation has at least 98.5% accuracy on the case of a circular arc, which is the
/// worst case for our standard shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LengthMeasurer {}

impl Measurer for LengthMeasurer {
    fn measure_cubic(&self, cubic: Cubic) -> f32 {
        Self::closest_progress_to(cubic, f32::INFINITY).1
    }

    fn find_cubic_cut_point(&self, cubic: Cubic, m: f32) -> f32 {
        Self::closest_progress_to(cubic, m).0
    }
}

impl LengthMeasurer {
    // The minimum number needed to achieve up to 98.5% accuracy from the true arc length
    // See PolygonMeasureTest.measureCircle
    const SEGMENTS: usize = 3;

    pub fn new() -> Self {
        Self {}
    }

    fn closest_progress_to(cubic: Cubic, threshold: f32) -> (f32, f32) {
        let mut total = 0.0;
        let mut remainder = threshold;
        let mut prev = cubic.anchor_0;

        for i in 1..Self::SEGMENTS {
            let progress = i as f32 / Self::SEGMENTS as f32;
            let point = cubic.point_on_curve(progress);
            let segment = distance(point - prev);

            if segment >= remainder {
                return (
                    progress - (1.0 - remainder / segment) / Self::SEGMENTS as f32,
                    threshold,
                );
            }

            remainder -= segment;
            total += segment;
            prev = point;
        }

        (1.0, total)
    }
}
