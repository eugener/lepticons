//! Lucide icons for Leptos applications.
//!
//! Provides [Lucide](https://lucide.dev) icons as a Leptos component.
//! Icons are grouped into 42 category features for selective compilation.
//!
//! # Example
//!
//! ```rust,ignore
//! use lepticons::{Icon, LucideGlyph};
//! use leptos::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! { <Icon glyph=LucideGlyph::Activity /> }
//! }
//! ```

mod lucide_icon_data;
mod lucide_icon_impl;

pub use lucide_icon_data::LucideGlyph;
pub use lucide_icon_impl::Glyph;

use leptos::prelude::*;

const DEFAULT_SIZE: &str = "24";
const DEFAULT_FILL: &str = "none";
const DEFAULT_STROKE: &str = "currentColor";
const DEFAULT_STROKE_WIDTH: &str = "1.5";

/// Renders an SVG icon.
///
/// # Props
/// - `glyph` - The icon to render (accepts `LucideGlyph` directly or a `Signal<LucideGlyph>`)
/// - `class` - CSS class (default: `""`)
/// - `size` - Width and height in pixels (default: `"24"`)
/// - `fill` - SVG fill color (default: `"none"`)
/// - `stroke` - SVG stroke color (default: `"currentColor"`)
/// - `stroke_width` - SVG stroke width (default: `"1.5"`)
#[component]
pub fn Icon(
    #[prop(into)] glyph: Signal<LucideGlyph>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = DEFAULT_SIZE)] size: &'static str,
    #[prop(default = DEFAULT_FILL)] fill: &'static str,
    #[prop(default = DEFAULT_STROKE)] stroke: &'static str,
    #[prop(default = DEFAULT_STROKE_WIDTH)] stroke_width: &'static str,
) -> impl IntoView {
    view! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class=class
          width=size
          height=size
          viewBox="0 0 24 24"
          fill=fill
          stroke=stroke
          stroke-width=stroke_width
          stroke-linecap="round"
          stroke-linejoin="round"
          inner_html=move || glyph.get().svg()
        />
    }
}
