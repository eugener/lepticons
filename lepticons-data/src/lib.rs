//! Lucide icon data, search, and categories.
//!
//! Framework-agnostic core for the Lepticons toolkit. Provides the
//! [`LucideGlyph`] enum, the [`Glyph`] trait, lookup helpers, a cached
//! search index, and a category aggregation cache.
//!
//! Re-exported through the `lepticons` crate; depend on this crate
//! directly only when you need the data layer without a renderer.

/// Version of the `lepticons-data` crate, taken from `Cargo.toml` at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod lucide_icon_data;
mod lucide_icon_impl;

pub use lucide_icon_data::LucideGlyph;
pub use lucide_icon_impl::Glyph;

/// Re-exports of the [`strum`](https://docs.rs/strum) traits implemented
/// by [`LucideGlyph`]. Importing from here lets downstream callers iterate
/// or look up icons without taking a direct `strum` dependency.
pub mod strum {
    pub use ::strum::{EnumProperty, IntoEnumIterator};
}
