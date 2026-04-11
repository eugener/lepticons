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

pub use draw_icon::DrawIcon;
pub use css_animations::AnimationStyles;
