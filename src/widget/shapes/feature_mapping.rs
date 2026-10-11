use iced::Point;

use crate::widget::shapes::{
    Error,
    features::{Feature, FeatureKind},
    float_mapping::{DoubleMapper, progress_distance, progress_in_range},
    morph::DISTANCE_EPSILON,
    utils::distance_squared,
};

const IDENTITY_MAPPING: [(f32, f32); 2] = [(0.0, 0.0), (0.5, 0.5)];

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressableFeature {
    pub progress: f32,
    pub feature: Feature,
}

fn feature_representative_point(feature: Feature) -> Point {
    // TODO: Error handling for features with no cubics maybe?
    let x = (feature.cubics.first().unwrap().anchor_0.x
        + feature.cubics.last().unwrap().anchor_1.x)
        / 2.0;
    let y = (feature.cubics.first().unwrap().anchor_0.y
        + feature.cubics.last().unwrap().anchor_1.y)
        / 2.0;

    Point::new(x, y)
}

/// Returns distance along overall shape between two Features on the two different shapes. This
/// information is used to determine how to map features (and the curves that make up those
/// features).
fn feature_dist_squared(f1: Feature, f2: Feature) -> f32 {
    if matches!(
        f1.kind,
        FeatureKind::ConvexCorner | FeatureKind::ConcaveCorner
    ) && matches!(
        f2.kind,
        FeatureKind::ConvexCorner | FeatureKind::ConcaveCorner
    ) && f1.kind != f2.kind
    {
        // Simple hack to force all features to map only to features of the same concavity, by
        // returning an infinitely large distance in that case
        return f32::MAX;
    }

    distance_squared(feature_representative_point(f1) - feature_representative_point(f2))
}

#[derive(Debug, Clone, PartialEq)]
struct DistanceVertex {
    distance: f32,
    f1: ProgressableFeature,
    f2: ProgressableFeature,
}

struct MappingHelper {
    mapping: Vec<(f32, f32)>,
    used_f1: Vec<ProgressableFeature>,
    used_f2: Vec<ProgressableFeature>,
}

impl MappingHelper {
    fn new() -> Self {
        Self {
            mapping: Vec::new(),
            used_f1: Vec::new(),
            used_f2: Vec::new(),
        }
    }

    fn add_mapping(
        &mut self,
        f1: ProgressableFeature,
        f2: ProgressableFeature,
    ) -> Result<(), Error> {
        // TODO: Can be sped up
        // We don't want to map the same feature twice.
        if self.used_f1.contains(&f1) || self.used_f2.contains(&f2) {
            return Ok(());
        }

        // Ret is sorted, find where we need to insert this new mapping.
        let insertion_index = match self
            .mapping
            .binary_search_by(|m| m.0.total_cmp(&f1.progress))
        {
            Ok(_) => return Err(Error::DuplicateProgressValue),
            Err(index) => index,
        };
        let n = self.mapping.len();

        // We can always add the first 1 element
        if n >= 1 {
            let (before1, before2) = self.mapping[(insertion_index + n - 1) % n];
            let (after1, after2) = self.mapping[insertion_index % n];

            // We don't want features that are way too close to each other, that will make the
            // DoubleMapper unstable
            if progress_distance(f1.progress, before1) < DISTANCE_EPSILON
                || progress_distance(f1.progress, after1) < DISTANCE_EPSILON
                || progress_distance(f2.progress, before2) < DISTANCE_EPSILON
                || progress_distance(f2.progress, after2) < DISTANCE_EPSILON
            {
                return Ok(());
            }

            // When we have 2 or more elements, we need to ensure we are not adding extra crossings.
            if n > 1 && !progress_in_range(f2.progress, before2, after2) {
                return Ok(());
            }
        }

        // All good, we can add the mapping.
        self.mapping
            .insert(insertion_index, (f1.progress, f2.progress));
        self.used_f1.push(f1);
        self.used_f2.push(f2);

        Ok(())
    }
}

/// Returns a mapping of the features between features1 and features2. The return is a list of pairs
/// in which the first element is the progress of a feature in features1 and the second element is
/// the progress of the feature in features2 that we mapped it to. The list is sorted by the first
/// element. To do this:
/// 1) Compute the distance for all pairs of features in (features1 x features2),
/// 2) Sort ascending by by such distance
/// 3) Try to add them, from smallest distance to biggest, ensuring that: a) The features we are
///    mapping haven't been mapped yet. b) We are not adding a crossing in the mapping. Since the
///    mapping is sorted by the first element of each pair, this means that the second elements of
///    each pair are monotonically increasing, except maybe one time (Counting all pair of
///    consecutive elements, and the last element to first element).
fn do_mapping(
    features1: Vec<ProgressableFeature>,
    features2: Vec<ProgressableFeature>,
) -> Result<Vec<(f32, f32)>, Error> {
    let mut distance_vertex_list = Vec::new();
    for f1 in &features1 {
        for f2 in &features2 {
            let d = feature_dist_squared(f1.feature.clone(), f2.feature.clone());
            if d != f32::MAX {
                distance_vertex_list.push(DistanceVertex {
                    distance: d,
                    f1: f1.clone(),
                    f2: f2.clone(),
                });
            }
        }
    }
    distance_vertex_list.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    if distance_vertex_list.is_empty() {
        return Ok(IDENTITY_MAPPING.to_vec());
    }
    if distance_vertex_list.len() == 1 {
        let first = distance_vertex_list.first().unwrap();
        let f1 = first.f1.progress;
        let f2 = first.f2.progress;
        return Ok(vec![(f1, f2), (((f1 + 0.5) % 1.0), ((f2 + 0.5) % 1.0))]);
    }

    let mut helper = MappingHelper::new();
    for distance_vertex in distance_vertex_list {
        helper.add_mapping(distance_vertex.f1.clone(), distance_vertex.f2.clone())?;
    }

    return Ok(helper.mapping);
}

/// Creates a mapping between the "features" (rounded corners) of two shapes.
pub fn feature_mapper(
    features1: Vec<ProgressableFeature>,
    features2: Vec<ProgressableFeature>,
) -> Result<DoubleMapper, Error> {
    let filtered_features_1 = features1
        .into_iter()
        .filter(|f| {
            matches!(f.feature.kind, FeatureKind::ConvexCorner)
                || matches!(f.feature.kind, FeatureKind::ConcaveCorner)
        })
        .collect();
    let filtered_features_2 = features2
        .into_iter()
        .filter(|f| {
            matches!(f.feature.kind, FeatureKind::ConvexCorner)
                || matches!(f.feature.kind, FeatureKind::ConcaveCorner)
        })
        .collect();

    let feature_progress_mapping = do_mapping(filtered_features_1, filtered_features_2)?;
    DoubleMapper::new(feature_progress_mapping)
}
