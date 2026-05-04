use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::{Icon, LucideGlyph};

use crate::theme;

const DEFAULT_STRIP_STYLE: &str = "padding:0.25rem 0.75rem 0.5rem;\
    border-bottom:1px solid var(--lp-border,#e5e5e5)";

const DEFAULT_HEADER_STYLE: &str = "font-size:0.6875rem;font-weight:500;letter-spacing:0.04em;\
    text-transform:uppercase;\
    color:var(--lp-text-muted,#999);\
    margin-bottom:0.25rem";

const DEFAULT_ITEMS_STYLE: &str = "display:flex;flex-wrap:wrap;gap:0.25rem";

const DEFAULT_ITEM_STYLE: &str = "padding:0.375rem;\
    border-radius:var(--lp-radius,0.5rem);\
    background:var(--lp-bg,#f5f5f5);\
    cursor:pointer;display:flex";

/// Horizontal strip of recently-used icons.
///
/// Reads from `mru` (an `RwSignal<Vec<LucideGlyph>>`) and emits `on_select`
/// when a user activates a cell with mouse or keyboard. Persistence and
/// pruning of the underlying `Vec` are caller responsibilities -- the picker
/// crate's [`crate::mru`] module provides `load`, `save`, and `push_into`
/// helpers backed by `localStorage`.
///
/// Styling follows the same opt-in pattern as
/// [`crate::IconGrid`]: when `class`, `header_class`, or `item_class` are
/// supplied, the corresponding default inline styles are suppressed so the
/// caller fully owns the look.
///
/// # Example
///
/// ```rust,ignore
/// use leptos::prelude::*;
/// use lepticons::LucideGlyph;
/// use lepticons_picker::{mru, MruStrip};
///
/// let mru_signal = RwSignal::new(mru::load("my-mru"));
/// let on_select = Callback::new(|_g: LucideGlyph| { /* ... */ });
///
/// view! { <MruStrip mru=mru_signal on_select=on_select /> }
/// ```
#[component]
#[allow(clippy::too_many_arguments)]
pub fn MruStrip(
    /// MRU list to render. Reactively re-renders when the contents change.
    mru: RwSignal<Vec<LucideGlyph>>,
    /// Called when a cell is clicked or activated via Enter/Space.
    on_select: Callback<LucideGlyph>,
    /// CSS class for the outer wrapper. When set, the default inline padding
    /// and bottom-border are suppressed.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// CSS class for the header label. When set, suppresses default header
    /// inline styles.
    #[prop(into, optional)]
    header_class: Option<TextProp>,
    /// CSS class for each cell. When set, suppresses default cell inline
    /// styles.
    #[prop(into, optional)]
    item_class: Option<TextProp>,
    /// CSS class for the inner `role="list"` container that holds the
    /// cells. When set, suppresses the default `flex flex-wrap` styling so
    /// the caller can opt for `flex-nowrap overflow-x-auto`, a CSS grid,
    /// etc.
    #[prop(into, optional)]
    items_class: Option<TextProp>,
    /// Header text (default: "Recently used"). Use `show_header=false` to hide.
    #[prop(into, optional)]
    header_text: Option<TextProp>,
    /// Whether to render the header label.
    #[prop(default = true)]
    show_header: bool,
    /// Icon size (default "20").
    #[prop(into, optional)]
    icon_size: Option<TextProp>,
    /// Icon stroke color (default "currentColor").
    #[prop(into, optional)]
    icon_stroke: Option<TextProp>,
    /// Icon stroke width (default "1.5").
    #[prop(into, optional)]
    icon_stroke_width: Option<TextProp>,
    /// Icon fill (default "none").
    #[prop(into, optional)]
    icon_fill: Option<TextProp>,
) -> impl IntoView {
    let header_text = header_text.unwrap_or_else(|| "Recently used".into());
    let icon_size = icon_size.unwrap_or_else(|| "20".into());
    let icon_stroke = icon_stroke.unwrap_or_else(|| "currentColor".into());
    let icon_stroke_width = icon_stroke_width.unwrap_or_else(|| "1.5".into());
    let icon_fill = icon_fill.unwrap_or_else(|| "none".into());

    view! {
        <div class={
                 let class = class.clone();
                 move || theme::class_str(&class)
             }
             style={
                 let class = class.clone();
                 move || theme::style_str(&class, DEFAULT_STRIP_STYLE)
             }>
            {show_header.then(|| {
                let header_class_for_class = header_class.clone();
                let header_class_for_style = header_class;
                view! {
                    <div class=move || theme::class_str(&header_class_for_class)
                         style=move || theme::style_str(&header_class_for_style, DEFAULT_HEADER_STYLE)>
                        {move || header_text.get().to_string()}
                    </div>
                }
            })}
            <div role="list"
                 class={
                     let items_class = items_class.clone();
                     move || theme::class_str(&items_class)
                 }
                 style={
                     let items_class = items_class.clone();
                     move || theme::style_str(&items_class, DEFAULT_ITEMS_STYLE)
                 }>
                {move || mru.get().into_iter().map(|icon| {
                    let label = icon.kebab_name();
                    let item_class_for_class = item_class.clone();
                    let item_class_for_style = item_class.clone();
                    let size = icon_size.clone();
                    let stroke = icon_stroke.clone();
                    let stroke_width = icon_stroke_width.clone();
                    let fill = icon_fill.clone();
                    view! {
                        <div role="listitem"
                             aria-label=label.clone()
                             title=label
                             class=move || theme::class_str(&item_class_for_class)
                             style=move || theme::style_str(&item_class_for_style, DEFAULT_ITEM_STYLE)
                             tabindex="0"
                             on:click=move |_| on_select.run(icon)
                             on:keydown=move |ev: web_sys::KeyboardEvent| {
                                 if ev.key() == "Enter" || ev.key() == " " {
                                     ev.prevent_default();
                                     on_select.run(icon);
                                 }
                             }>
                            <Icon glyph=icon
                                  size=move || size.get()
                                  stroke=move || stroke.get()
                                  stroke_width=move || stroke_width.get()
                                  fill=move || fill.get()
                            />
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
