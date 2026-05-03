//! Lucide icon data, search, and categories.
//!
//! Framework-agnostic core for the Lepticons toolkit. Provides the
//! [`LucideGlyph`] enum, the [`Glyph`] trait, lookup helpers, a cached
//! search index, and a category aggregation cache.
//!
//! Re-exported through the `lepticons` crate; depend on this crate
//! directly only when you need the data layer without a renderer.

mod lucide_icon_data;
mod lucide_icon_impl;

pub use lucide_icon_data::LucideGlyph;
pub use lucide_icon_impl::Glyph;
