//! Icon animations for Leptos -- stroke draw-in, spin, pulse, bounce.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use lepticons::LucideGlyph;
//! use lepticons_animate::DrawIcon;
//!
//! // Stroke draw-in animation
//! <DrawIcon glyph=LucideGlyph::Check duration_ms=500 />
//!
//! // CSS utility animations (use class on the standard Icon component)
//! use lepticons::Icon;
//! <Icon glyph=LucideGlyph::Loader class="lepticons-spin" />
//! <Icon glyph=LucideGlyph::Bell class="lepticons-bounce" />
//! ```

mod draw_icon;
mod css_animations;
mod easing;

pub use draw_icon::DrawIcon;
pub use css_animations::AnimationStyles;
pub use easing::Easing;

/// Version of the `lepticons-animate` crate, taken from `Cargo.toml` at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
