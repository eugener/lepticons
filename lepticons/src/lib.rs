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
use leptos::text_prop::TextProp;

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
///
/// All string props accept static strings, `String`, signals, or closures:
/// ```rust,ignore
/// // Static
/// <Icon glyph=LucideGlyph::Search size="32" />
/// // Reactive
/// <Icon glyph=LucideGlyph::Heart class=move || if active.get() { "text-red-500" } else { "" } />
/// ```
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)] glyph: Signal<LucideGlyph>,
    /// CSS class for the SVG element.
    #[prop(into, optional)] class: Option<TextProp>,
    /// Width and height in pixels (default: `"24"`).
    #[prop(into, optional)] size: Option<TextProp>,
    /// SVG fill color (default: `"none"`).
    #[prop(into, optional)] fill: Option<TextProp>,
    /// SVG stroke color (default: `"currentColor"`).
    #[prop(into, optional)] stroke: Option<TextProp>,
    /// SVG stroke width (default: `"1.5"`).
    #[prop(into, optional)] stroke_width: Option<TextProp>,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "".into());
    let size = size.unwrap_or_else(|| DEFAULT_SIZE.into());
    let size2 = size.clone();
    let fill = fill.unwrap_or_else(|| DEFAULT_FILL.into());
    let stroke = stroke.unwrap_or_else(|| DEFAULT_STROKE.into());
    let stroke_width = stroke_width.unwrap_or_else(|| DEFAULT_STROKE_WIDTH.into());

    view! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class=move || class.get()
          width=move || size.get()
          height=move || size2.get()
          viewBox="0 0 24 24"
          fill=move || fill.get()
          stroke=move || stroke.get()
          stroke-width=move || stroke_width.get()
          stroke-linecap="round"
          stroke-linejoin="round"
          inner_html=move || glyph.get().svg()
        />
    }
}
