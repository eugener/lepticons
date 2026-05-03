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

/// Version of the `lepticons` crate, taken from `Cargo.toml` at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default icon size in pixels.
pub const DEFAULT_SIZE: &str = "24";
/// Default SVG fill color.
pub const DEFAULT_FILL: &str = "none";
/// Default SVG stroke color.
pub const DEFAULT_STROKE: &str = "currentColor";
/// Default SVG stroke width.
pub const DEFAULT_STROKE_WIDTH: &str = "1.5";

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
    let svg = TextProp::from(move || glyph.get().svg());
    render_svg(svg, class, size, fill, stroke, stroke_width)
}

/// Renders an arbitrary inline SVG using the same prop API as [`Icon`].
///
/// Use for in-house icons, brand logos, or icons missing from Lucide.
/// The `svg` prop is the inner SVG markup (paths, circles, etc.) without
/// the wrapping `<svg>` element. Content is expected to be authored against
/// a `0 0 24 24` viewBox to match Lucide's defaults.
///
/// # Props
/// - `svg` - Inner SVG markup (required, accepts static strings, `String`, signals, or closures)
/// - `class` - CSS class (default: `""`)
/// - `size` - Width and height in pixels (default: `"24"`)
/// - `fill` - SVG fill color (default: `"none"`)
/// - `stroke` - SVG stroke color (default: `"currentColor"`)
/// - `stroke_width` - SVG stroke width (default: `"1.5"`)
///
/// # Example
///
/// ```rust,ignore
/// use lepticons::CustomIcon;
/// use leptos::prelude::*;
///
/// const COMPANY_LOGO: &str = r#"<path d="M12 2L2 7l10 5 10-5-10-5z" />"#;
///
/// view! {
///     <CustomIcon svg=COMPANY_LOGO class="text-primary" size="32" />
/// }
/// ```
///
/// # Security
///
/// The `svg` prop is rendered as raw HTML via `inner_html`. SVG can carry
/// executable content -- `<script>`, `<foreignObject>`/`<iframe>`, and
/// element event handlers (`onload="..."`) all run inside the host page's
/// origin. Pass only trusted, author-controlled markup (build-time
/// constants, vendored assets, or strings you fully sanitize). Never feed
/// `CustomIcon` strings sourced from end users, the network, or any
/// upload/CMS pipeline without first running them through an SVG-aware
/// sanitizer.
#[component]
pub fn CustomIcon(
    /// Inner SVG markup (paths, circles, etc.) authored against a `0 0 24 24` viewBox.
    #[prop(into)] svg: TextProp,
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
    render_svg(svg, class, size, fill, stroke, stroke_width)
}

fn render_svg(
    svg: TextProp,
    class: Option<TextProp>,
    size: Option<TextProp>,
    fill: Option<TextProp>,
    stroke: Option<TextProp>,
    stroke_width: Option<TextProp>,
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
          inner_html=move || svg.get()
        />
    }
}
