//! Numeric definitions copied from the official Material Web ripple.
//!
//! Source: <https://github.com/material-components/material-web>, commit `d872455`.
//! `ripple.ts` is `ripple/internal/ripple.ts`, `_ripple.scss` is
//! `ripple/internal/_ripple.scss` and `_md-sys-state.scss` is
//! `internal/_md-sys-state.scss` (line numbers refer to that commit).

use std::time::Duration;

/// Duration of the grow animation. `ripple.ts:17`
pub const PRESS_GROW: Duration = Duration::from_millis(450);
/// Minimum time the press layer stays visible. `ripple.ts:18`
pub const MINIMUM_PRESS: Duration = Duration::from_millis(225);
/// Initial size of the ripple relative to the larger container side. `ripple.ts:19`
pub const INITIAL_ORIGIN_SCALE: f32 = 0.2;
/// Extra radius added to the circumscribed circle. `ripple.ts:20`
pub const PADDING: f32 = 10.0;
/// Minimum width of the soft edge. `ripple.ts:21`
pub const SOFT_EDGE_MINIMUM_SIZE: f32 = 75.0;
/// Soft edge width relative to the larger container side. `ripple.ts:22`
pub const SOFT_EDGE_CONTAINER_RATIO: f32 = 0.35;
/// Delay before a touch press starts the animation. `ripple.ts:88`
pub const TOUCH_DELAY: Duration = Duration::from_millis(150);
/// Hover layer fade (linear). `_ripple.scss:67`
pub const HOVER_FADE: Duration = Duration::from_millis(15);
/// Press layer fade-in (linear). `_ripple.scss:90`
pub const PRESS_FADE_IN: Duration = Duration::from_millis(105);
/// Press layer fade-out (linear). `_ripple.scss:78`
pub const PRESS_FADE_OUT: Duration = Duration::from_millis(375);
/// Width of the gradient feather in pixels. `_ripple.scss:74`
pub const GRADIENT_FEATHER_PX: f32 = 70.0;
/// Minimum fraction of the radius that is fully opaque. `_ripple.scss:74`
pub const GRADIENT_SOLID_MIN_RATIO: f32 = 0.65;

/// Number of concentric rings used to approximate the radial gradient.
///
/// Equal to `round(0.12 * 255)`, the number of distinct 8-bit opacity steps of
/// the pressed state layer, so the banding stays within one step.
pub const GRADIENT_RINGS: usize = 31;
/// Maximum distance between a polygonised arc and the true curve in pixels.
pub const CHORD_TOLERANCE: f32 = 0.1;
