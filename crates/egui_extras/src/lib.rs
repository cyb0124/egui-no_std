//! This is a crate that adds some features on top top of [`egui`](https://github.com/emilk/egui).
//!
//! This crate are for experimental features, and features that require big dependencies that does not belong in `egui`.
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

#![allow(clippy::float_cmp)]
#![allow(clippy::manual_range_contains)]
#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

pub mod syntax_highlighting;

#[doc(hidden)]
pub mod image;
mod layout;
mod loaders;
mod sizing;
mod strip;
mod table;

#[doc(hidden)]
#[allow(deprecated)]
pub use crate::image::RetainedImage;
pub(crate) use crate::layout::StripLayout;
pub use crate::sizing::Size;
pub use crate::strip::*;
pub use crate::table::*;

pub use loaders::install_image_loaders;

/// Panic in debug builds, log otherwise.
macro_rules! log_or_panic {
    ($fmt: literal) => {$crate::log_or_panic!($fmt,)};
    ($fmt: literal, $($arg: tt)*) => {{
        if cfg!(debug_assertions) {
            panic!($fmt, $($arg)*);
        }
    }};
}
pub(crate) use log_or_panic;
