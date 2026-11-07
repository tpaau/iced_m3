//! Ripple effect, ported from the Material Web ripple.
//!
//! The module is split in the Elm style. Everything except [`render`] and
//! [`binding`] is pure; time is always passed in as an argument.
//!
//! - [`constant`]: numbers copied from the official implementation
//! - [`model`]: the state ([`Model`], [`Phase`], [`Press`], [`Transition`])
//! - [`message`]: events the model reacts to
//! - [`update`](mod@update): state transitions
//! - [`view`](mod@view): [`Model`] to [`Scene`]
//! - [`render`]: draws a [`Scene`]
//! - [`binding`]: translates iced events, see [`Ripple`]
//!
//! Intentional differences from the official implementation:
//!
//! - A touch that is released outside of the bounds cancels the press.
//! - Nothing is drawn if the initial ripple size is zero (containers smaller
//!   than 5 px).
//! - The radial gradient is approximated by [`constant::GRADIENT_RINGS`] rings.
//! - The zoom is always `1`, scaling is handled by the renderer.

pub mod binding;
pub mod clip;
pub mod constant;
pub mod message;
pub mod model;
pub mod render;
pub mod update;
pub mod view;

pub use binding::Ripple;
pub use message::Message;
pub use model::{Model, Phase, Pointer, Press, Transition};
pub use update::{Context, Effect, update};
pub use view::{Disc, Scene, view};

#[cfg(test)]
mod tests;
