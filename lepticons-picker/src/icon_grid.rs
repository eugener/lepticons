use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos::wasm_bindgen::JsCast;
use lepticons::{Icon, LucideGlyph};

use crate::copy::{copy_to_clipboard, IconCopyFormat};

/// Internal context provided by [`IconGrid`] when a `copy_format` signal is
/// supplied. Cells use it to render the per-cell hover copy button.
#[derive(Copy, Clone)]
struct CopyContext {
    format: Signal<IconCopyFormat>,
    last_copied: RwSignal<Option<LucideGlyph>>,
}

/// Grid of icon cells with selection, keyboard navigation, and tooltip support.
///
/// Filters icons using [`LucideGlyph::find()`] and lays them out in a CSS grid
/// (`grid-template-columns: repeat(auto-fill, minmax(2.5rem, 1fr))`).
/// When a `class` is supplied the inline grid style is suppressed so callers
/// can fully own the layout.
///
/// Internally uses a keyed `<For>` so cells that survive a filter change are
/// reused rather than remounted.
///
/// # Keyboard
///
/// - Arrow keys move focus within the visible grid
/// - Home / End jump to the first / last visible cell
/// - PageUp / PageDown move by ~5 rows
/// - Enter (or Space) selects the focused cell
///
/// Cells use a roving `tabindex` so the grid takes a single tab stop. Focus
/// changes triggered by keyboard nav scroll the focused cell into view.
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
    /// Called when an icon cell is clicked or activated via Enter/Space.
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
    /// Stroke color for icons in the grid (default: "currentColor").
    #[prop(into, optional)]
    icon_stroke: Option<TextProp>,
    /// Stroke width for icons in the grid (default: "1.5").
    #[prop(into, optional)]
    icon_stroke_width: Option<TextProp>,
    /// Fill color for icons in the grid (default: "none").
    #[prop(into, optional)]
    icon_fill: Option<TextProp>,
    /// CSS class for the per-cell tooltip. When provided, replaces the default
    /// inline tooltip style.
    #[prop(into, optional)]
    tooltip_class: Option<TextProp>,
    /// Whether to show icon name tooltips on hover/focus.
    #[prop(default = true)]
    tooltips: bool,
    /// When supplied, each cell renders a small hover/focus copy button that
    /// writes the icon code (in this format) to the clipboard. Leave `None`
    /// to disable the copy affordance entirely.
    #[prop(into, optional)]
    copy_format: Option<Signal<IconCopyFormat>>,
) -> impl IntoView {
    let icon_size = icon_size.unwrap_or_else(|| "24".into());
    let icon_stroke = icon_stroke.unwrap_or_else(|| "currentColor".into());
    let icon_stroke_width = icon_stroke_width.unwrap_or_else(|| "1.5".into());
    let icon_fill = icon_fill.unwrap_or_else(|| "none".into());

    let grid_style = "display:grid;\
        grid-template-columns:repeat(auto-fill,minmax(2.5rem,1fr));\
        gap:0.5rem";

    let has_class = class.is_some();
    let has_cell_class = cell_class.is_some();
    let has_cell_selected_class = cell_selected_class.is_some();
    let has_tooltip_class = tooltip_class.is_some();

    let inject_default_tooltip_style = !has_tooltip_class && tooltips;
    let copy_enabled = copy_format.is_some();
    if let Some(format) = copy_format {
        provide_context(CopyContext {
            format,
            last_copied: RwSignal::new(None),
        });
    }

    // Roving focus tracked by glyph (not index) so a keyed `<For>` can reuse
    // cells across filter changes without losing focus state.
    let focused: RwSignal<Option<LucideGlyph>> = RwSignal::new(None);
    let grid_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Cached filter result. Memo's PartialEq comparison short-circuits
    // dependents when the result is unchanged.
    let filtered = Memo::new(move |_| LucideGlyph::find(&filter.get()));

    // Reset focus to the first visible icon when the filter result changes.
    Effect::new(move |_| {
        let first = filtered.with(|v| v.first().copied());
        focused.set(first);
    });

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let n = filtered.with(|v| v.len());
        if n == 0 {
            return;
        }
        let cur = filtered.with(|v| {
            focused
                .get_untracked()
                .and_then(|g| v.iter().position(|x| *x == g))
                .unwrap_or(0)
        });
        let cols = grid_ref
            .get()
            .map(|el| columns_count(&el))
            .unwrap_or(1)
            .max(1);
        let new_idx = match ev.key().as_str() {
            "ArrowLeft" => cur.saturating_sub(1),
            "ArrowRight" => (cur + 1).min(n - 1),
            "ArrowUp" => cur.saturating_sub(cols),
            "ArrowDown" => (cur + cols).min(n - 1),
            "Home" => 0,
            "End" => n - 1,
            "PageUp" => cur.saturating_sub(cols * 5),
            "PageDown" => (cur + cols * 5).min(n - 1),
            "Enter" | " " => {
                if let Some(icon) = filtered.with(|v| v.get(cur).copied()) {
                    on_select.run(icon);
                }
                ev.prevent_default();
                return;
            }
            _ => return,
        };
        ev.prevent_default();
        if let Some(glyph) = filtered.with(|v| v.get(new_idx).copied()) {
            focused.set(Some(glyph));
        }
        if let Some(grid_el) = grid_ref.get() {
            if let Some(child) = grid_el.children().item(new_idx as u32) {
                if let Some(html) = child.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = html.focus();
                    let opts = web_sys::ScrollIntoViewOptions::new();
                    opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
                    html.scroll_into_view_with_scroll_into_view_options(&opts);
                }
            }
        }
    };

    view! {
        {inject_default_tooltip_style.then(|| view! {
            <style>
                ".lp-cell:hover .lp-tooltip,\
                 .lp-cell:focus-visible .lp-tooltip,\
                 .lp-cell:focus .lp-tooltip{opacity:1!important}"
            </style>
        })}
        {copy_enabled.then(|| view! {
            <style>
                ".lp-cell .lp-copy{opacity:0;transition:opacity 0.12s}\
                 .lp-cell:hover .lp-copy,\
                 .lp-cell:focus-within .lp-copy,\
                 .lp-cell:focus-visible .lp-copy{opacity:1}"
            </style>
        })}
        <div node_ref=grid_ref
             class=move || class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
             style=move || if has_class { "" } else { grid_style }
             role="grid"
             aria-label="Icons"
             on:keydown=on_keydown>
            <For
                each=move || filtered.get()
                key=|icon| *icon
                let:icon
            >
                {
                    let size = icon_size.clone();
                    let stroke = icon_stroke.clone();
                    let stroke_width = icon_stroke_width.clone();
                    let fill = icon_fill.clone();
                    let cell_class = cell_class.clone();
                    let cell_selected_class = cell_selected_class.clone();
                    let tooltip_class = tooltip_class.clone();
                    let is_selected = Signal::derive(move || selected.get() == Some(icon));
                    let is_focused = Signal::derive(move || focused.get() == Some(icon));
                    view! {
                        <IconCell
                            icon=icon
                            selected=is_selected
                            is_focused=is_focused
                            focused=focused
                            on_select=on_select
                            size=size
                            stroke=stroke
                            stroke_width=stroke_width
                            fill=fill
                            tooltips=tooltips
                            has_cell_class=has_cell_class
                            has_cell_selected_class=has_cell_selected_class
                            cell_class=cell_class
                            cell_selected_class=cell_selected_class
                            has_tooltip_class=has_tooltip_class
                            tooltip_class=tooltip_class
                        />
                    }
                }
            </For>
        </div>
        {move || {
            let f = filter.get();
            let empty = filtered.with(|v| v.is_empty());
            (empty && !f.is_empty()).then(|| view! {
                <div role="status"
                     aria-live="polite"
                     style="padding:1.5rem 1rem;text-align:center;\
                            color:var(--lp-text-muted,#999);\
                            font-size:0.875rem;line-height:1.6">
                    "No icons match \""
                    <span style="color:var(--lp-text,inherit);font-weight:500">{f}</span>
                    "\". Search uses name, tags, and category."
                    <br/>
                    "Missing an icon? "
                    <a href="https://github.com/lucide-icons/lucide/issues/new?template=icon_request.yml"
                       target="_blank"
                       rel="noreferrer"
                       style="color:var(--lp-link,inherit);text-decoration:underline">
                        "Request it on lucide-icons/lucide"
                    </a>
                    "."
                </div>
            })
        }}
    }
}

/// Counts visible columns by walking children until the row offsetTop changes.
fn columns_count(el: &web_sys::HtmlDivElement) -> usize {
    let children = el.children();
    let len = children.length();
    if len == 0 {
        return 1;
    }
    let Some(first) = children.item(0) else {
        return 1;
    };
    let Some(first_html) = first.dyn_ref::<web_sys::HtmlElement>() else {
        return 1;
    };
    let first_top = first_html.offset_top();
    for i in 1..len {
        if let Some(child) = children.item(i) {
            if let Some(html) = child.dyn_ref::<web_sys::HtmlElement>() {
                if html.offset_top() != first_top {
                    return i as usize;
                }
            }
        }
    }
    len as usize
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
#[allow(clippy::too_many_arguments)]
fn IconCell(
    icon: LucideGlyph,
    selected: Signal<bool>,
    is_focused: Signal<bool>,
    focused: RwSignal<Option<LucideGlyph>>,
    on_select: Callback<LucideGlyph>,
    size: TextProp,
    stroke: TextProp,
    stroke_width: TextProp,
    fill: TextProp,
    tooltips: bool,
    has_cell_class: bool,
    has_cell_selected_class: bool,
    cell_class: Option<TextProp>,
    cell_selected_class: Option<TextProp>,
    has_tooltip_class: bool,
    tooltip_class: Option<TextProp>,
) -> impl IntoView {
    let on_click = move |_: web_sys::MouseEvent| {
        focused.set(Some(icon));
        on_select.run(icon);
        // No scrollIntoView on click: the user just clicked the cell, so
        // it's already in view by definition. Auto-scrolling here would
        // jump the grid (e.g. centering the cell) and feel like the page
        // lost its top portion.
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        if focused.get_untracked() != Some(icon) {
            focused.set(Some(icon));
        }
    };

    let class_fn = move || {
        let custom = if has_cell_class || has_cell_selected_class {
            if selected.get() {
                cell_selected_class
                    .as_ref()
                    .map(|c| c.get().to_string())
                    .unwrap_or_default()
            } else {
                cell_class
                    .as_ref()
                    .map(|c| c.get().to_string())
                    .unwrap_or_default()
            }
        } else {
            String::new()
        };
        format!("lp-cell {custom}")
    };

    let style_fn = move || {
        if has_cell_class || has_cell_selected_class {
            ""
        } else if selected.get() {
            DEFAULT_CELL_HOVER
        } else {
            DEFAULT_CELL_STYLE
        }
    };

    let aria_label = icon.kebab_name();
    let tabindex_fn = move || if is_focused.get() { "0" } else { "-1" };

    let copy_ctx = use_context::<CopyContext>();

    view! {
        <div class=class_fn
             style=style_fn
             role="gridcell"
             tabindex=tabindex_fn
             aria-label=aria_label
             aria-selected=move || selected.get().to_string()
             on:click=on_click
             on:focus=on_focus>
            <Icon glyph=icon
                  size=move || size.get()
                  stroke=move || stroke.get()
                  stroke_width=move || stroke_width.get()
                  fill=move || fill.get()
            />
            {tooltips.then(|| {
                let name = icon.name();
                let tooltip_class = tooltip_class.clone();
                let class_fn = move || {
                    if has_tooltip_class {
                        tooltip_class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
                    } else {
                        "lp-tooltip".to_string()
                    }
                };
                let style_fn = move || {
                    if has_tooltip_class { "" } else { DEFAULT_TOOLTIP_STYLE }
                };
                view! {
                    <div class=class_fn style=style_fn>{name}</div>
                }
            })}
            {copy_ctx.map(|ctx| {
                let on_copy = move |ev: web_sys::MouseEvent| {
                    ev.stop_propagation();
                    let format = ctx.format.get();
                    copy_to_clipboard(&format.render(icon));
                    ctx.last_copied.set(Some(icon));
                    set_timeout(
                        move || {
                            if ctx.last_copied.get_untracked() == Some(icon) {
                                ctx.last_copied.set(None);
                            }
                        },
                        std::time::Duration::from_millis(1500),
                    );
                };
                let copied = ctx.last_copied;
                let glyph_signal = Signal::derive(move || {
                    if copied.get() == Some(icon) {
                        LucideGlyph::Check
                    } else {
                        LucideGlyph::Copy
                    }
                });
                view! {
                    <button class="lp-copy"
                            type="button"
                            aria-label="Copy icon code"
                            title="Copy"
                            style=DEFAULT_COPY_BUTTON_STYLE
                            on:click=on_copy>
                        <Icon glyph=glyph_signal size="12" stroke="currentColor" stroke_width="2" />
                    </button>
                }
            })}
        </div>
    }
}

const DEFAULT_COPY_BUTTON_STYLE: &str = "\
    position:absolute;top:2px;right:2px;\
    display:flex;align-items:center;justify-content:center;\
    width:1.125rem;height:1.125rem;padding:0;\
    color:var(--lp-text-muted,#999);\
    background:var(--lp-copy-bg,rgba(255,255,255,0.85));\
    border:1px solid var(--lp-border,rgba(0,0,0,0.08));\
    border-radius:0.25rem;cursor:pointer;\
    line-height:1";
