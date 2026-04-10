use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos::wasm_bindgen::JsCast;
use lepticons::{Icon, LucideGlyph};

/// Grid of icon cells with selection and tooltip support.
///
/// Filters icons using [`LucideGlyph::find()`] and displays them in a flex-wrap grid.
/// Clicking an icon invokes `on_select`.
///
/// # Example
///
/// ```rust,ignore
/// let (selected, set_selected) = signal(None::<LucideGlyph>);
/// <IconGrid
///     filter=filter_signal
///     selected=selected
///     on_select=Callback::new(move |icon| set_selected.set(Some(icon)))
/// />
/// ```
#[component]
pub fn IconGrid(
    /// Filter string (searched against name, tags, categories via LucideGlyph::find).
    #[prop(into)]
    filter: Signal<String>,
    /// Currently selected icon (used for highlight).
    #[prop(into)]
    selected: Signal<Option<LucideGlyph>>,
    /// Called when an icon cell is clicked.
    on_select: Callback<LucideGlyph>,
    /// CSS class for the grid container div.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// CSS class for each unselected icon cell.
    #[prop(into, optional)]
    cell_class: Option<TextProp>,
    /// CSS class for the selected icon cell.
    #[prop(into, optional)]
    cell_selected_class: Option<TextProp>,
    /// Icon size in the grid cells (default: "24").
    #[prop(into, optional)]
    icon_size: Option<TextProp>,
    /// Whether to show icon name tooltips on hover.
    #[prop(default = true)]
    tooltips: bool,
) -> impl IntoView {
    let icon_size = icon_size.unwrap_or_else(|| "24".into());

    let grid_style = "display:flex;flex-wrap:wrap;gap:0.5rem";

    let has_class = class.is_some();
    let has_cell_class = cell_class.is_some();
    let has_cell_selected_class = cell_selected_class.is_some();

    view! {
        <style>".lp-cell:hover .lp-tooltip{opacity:1!important}"</style>
        <div class=move || class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
             style=move || if has_class { "" } else { grid_style }>
        {
            move || {
                let filtered = LucideGlyph::find(filter.get().to_lowercase().as_str());
                filtered.into_iter().map(|icon| {
                    let is_selected = Signal::derive(move || selected.get() == Some(icon));
                    let size = icon_size.clone();
                    view! {
                        <IconCell
                            icon=icon
                            selected=is_selected
                            on_select=on_select
                            size=size
                            tooltips=tooltips
                            has_cell_class=has_cell_class
                            has_cell_selected_class=has_cell_selected_class
                            cell_class=cell_class.clone()
                            cell_selected_class=cell_selected_class.clone()
                        />
                    }
                }).collect::<Vec<_>>()
            }
        }
        </div>
    }
}

const DEFAULT_CELL_STYLE: &str = "\
    position:relative;\
    padding:0.5rem;\
    border-radius:var(--lp-radius,0.5rem);\
    border:1px solid transparent;\
    background:var(--lp-bg,#f5f5f5);\
    cursor:pointer";

const DEFAULT_CELL_HOVER: &str = "\
    position:relative;\
    padding:0.5rem;\
    border-radius:var(--lp-radius,0.5rem);\
    border:1px solid var(--lp-border-selected,rgba(192,58,23,0.8));\
    background:var(--lp-bg-selected,rgba(192,58,23,0.1));\
    cursor:pointer";

const DEFAULT_TOOLTIP_STYLE: &str = "\
    position:absolute;\
    left:50%;transform:translateX(-50%);\
    bottom:-1.25rem;z-index:10;\
    opacity:0;transition:opacity 0.15s;\
    padding:0.125rem 0.375rem;\
    font-size:0.625rem;font-weight:300;\
    color:var(--lp-tooltip-text,#fff);\
    background:var(--lp-tooltip-bg,rgba(192,58,23,0.9));\
    border-radius:0.25rem;white-space:nowrap;\
    pointer-events:none";

#[component]
fn IconCell(
    icon: LucideGlyph,
    selected: Signal<bool>,
    on_select: Callback<LucideGlyph>,
    size: TextProp,
    tooltips: bool,
    has_cell_class: bool,
    has_cell_selected_class: bool,
    cell_class: Option<TextProp>,
    cell_selected_class: Option<TextProp>,
) -> impl IntoView {
    let on_click = move |ev: web_sys::MouseEvent| {
        on_select.run(icon);
        if let Some(target) = ev.current_target() {
            if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                let opts = web_sys::ScrollIntoViewOptions::new();
                opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                opts.set_block(web_sys::ScrollLogicalPosition::Center);
                el.scroll_into_view_with_scroll_into_view_options(&opts);
            }
        }
    };

    let class_fn = move || {
        let custom = if has_cell_class || has_cell_selected_class {
            if selected.get() {
                cell_selected_class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
            } else {
                cell_class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
            }
        } else {
            String::new()
        };
        format!("lp-cell {custom}")
    };

    // Inline style mode (default theming)
    let style_fn = move || {
        if has_cell_class || has_cell_selected_class {
            ""
        } else if selected.get() {
            DEFAULT_CELL_HOVER
        } else {
            DEFAULT_CELL_STYLE
        }
    };

    view! {
        <div class=class_fn style=style_fn on:click=on_click>
            <Icon glyph=icon size=move || size.get() />
            {tooltips.then(|| {
                let name = icon.name();
                view! {
                    <div style=DEFAULT_TOOLTIP_STYLE class="lp-tooltip">{name}</div>
                }
            })}
        </div>
    }
}
