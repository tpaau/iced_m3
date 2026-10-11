use crate::widget::shapes::{Error, morph::DISTANCE_EPSILON, utils::positive_modulo};

/// Checks if the given progress is in the given progress range, since progress is in the [0..1)
/// interval, and wraps, there is a special case when progressTo < progressFrom. For example, if the
/// progress range is 0.7 to 0.2, both 0.8 and 0.1 are inside and 0.5 is outside.
pub fn progress_in_range(progress: f32, progress_from: f32, progress_to: f32) -> bool {
    if progress_to >= progress_from {
        progress >= progress_from && progress <= progress_to
    } else {
        progress >= progress_from || progress <= progress_to
    }
}

/// Maps from one set of progress values to another. This is used by [`DoubleMapper`] to retrieve
/// the value on one shape that maps to the appropriate value on the other.
fn linear_map(x_values: &[f32], y_values: &[f32], x: f32) -> Result<f32, Error> {
    if !(0.0..=1.0).contains(&x) {
        return Err(Error::InvalidProgress);
    }

    let segment_start_index = match (0..x_values.len())
        .find(|&i| progress_in_range(x, x_values[i], x_values[(i + 1) % x_values.len()]))
    {
        Some(index) => index,
        None => return Err(Error::NoSegmentContainsProgress),
    };

    let segment_end_index = (segment_start_index + 1) % x_values.len();
    let segment_size_x = positive_modulo(
        x_values[segment_end_index] - x_values[segment_start_index],
        1.0,
    );
    let segment_size_y = positive_modulo(
        y_values[segment_end_index] - y_values[segment_start_index],
        1.0,
    );

    let position_in_segment = if segment_size_x < 0.001 {
        0.5
    } else {
        positive_modulo(x - x_values[segment_start_index], 1.0) / segment_size_x
    };

    Ok(positive_modulo(
        y_values[segment_start_index] + segment_size_y * position_in_segment,
        1.0,
    ))
}

/// Distance between two progress values.
/// Since progress wraps around, we consider a difference of 0.99 as a distance of 0.01
pub fn progress_distance(p1: f32, p2: f32) -> f32 {
    let val = (p1 - p2).abs();
    val.min(1.0 - val)
}

/// Verify that a list of progress values are all in the range [0.0, 1.0) and is monotonically
/// increasing, with the exception of maybe one time in which the progress wraps around, this check
/// needs to include all pairs of consecutive elements in the list plus the last to first element
/// pair.
/// For example: (0.0, 0.3, 0.6) is a valid list, so are (0.3, 0.6, 0.0) and (0.6, 0.3, 0.0).
/// On the other hand, something like (0.5, 0.0, 0.7) is not (since it goes down twice, from 0.5 to
/// 0.0 and then from to 0.7 to 0.5).
fn validate_progress(p: &[f32]) -> Result<(), Error> {
    let mut prev = match p.last() {
        Some(v) => v,
        None => return Err(Error::EmptyProgress),
    };
    let mut wraps = 0;
    for curr in p {
        if !(0.0..=1.0).contains(curr) {
            return Err(Error::InvalidRange);
        }
        if progress_distance(*curr, *prev) <= DISTANCE_EPSILON {
            return Err(Error::DuplicateProgressValue);
        }
        if curr < prev {
            wraps += 1;
            if wraps > 1 {
                return Err(Error::ProgressDoubleWrap);
            }
        }
        prev = curr
    }

    Ok(())
}

/// DoubleMapper creates mappings from values in the
/// [0..1) source space to values in the [0..1) target space, and back. This mapping is created given a finite list of representative mappings, and this is extended to the whole interval by linear interpolation, and wrapping around. For example, if we have mappings 0.2 to 0.5 and 0.4 to 0.6, then 0.3 (which is in the middle of the source interval) will be mapped to 0.55 (the middle of the targets for the interval), 0.21 will map to 0.505, and so on. As a more complete example, if we use x to represent a value in the source space and y for the target space, and given as input the mappings 0 to 0, 0.5 to 0.25, this will create a mapping that: { if x in [0 .. 0.5]
/// } y = x / 2 { if x in [0.5 .. 1] } y = 0.25 + (x - 0.5) * 1.5 = x * 1.5 - 0.5 The mapping can
/// also be used the other way around (using the mapBack function), resulting in: { if y in
/// [0 .. 0.25] } x = y * 2 { if y in [0.25 .. 1] } x = (y + 0.5) / 1.5 This is used to create
/// mappings of progress values between the start and end shape, which is then used to insert new
/// curves and match curves overall.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DoubleMapper {
    source_values: Vec<f32>,
    target_values: Vec<f32>,
}

impl DoubleMapper {
    pub fn new(mappings: Vec<(f32, f32)>) -> Result<Self, Error> {
        let (source_values, target_values): (Vec<f32>, Vec<f32>) = mappings.into_iter().unzip();
        validate_progress(&source_values)?;
        validate_progress(&target_values)?;

        Ok(Self {
            source_values,
            target_values,
        })
    }

    pub fn map(&self, x: f32) -> Result<f32, Error> {
        linear_map(&self.source_values, &self.target_values, x)
    }

    pub fn map_back(&self, x: f32) -> Result<f32, Error> {
        linear_map(&self.target_values, &self.source_values, x)
    }
}
