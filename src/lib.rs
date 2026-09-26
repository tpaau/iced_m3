#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod animation;
pub mod style;
pub mod theme;
pub mod widget;

#[cfg(feature = "expressive-defaults")]
pub(crate) const EXPRESSIVE: bool = true;
#[cfg(not(feature = "expressive-defaults"))]
pub(crate) const EXPRESSIVE: bool = false;
