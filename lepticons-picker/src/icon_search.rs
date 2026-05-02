use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::{Icon, LucideGlyph};

const DEFAULT_CONTAINER_STYLE: &str = "display:flex;align-items:center;gap:0.5rem;\
    padding:0.5rem 1rem;\
    background:var(--lp-bg,#f5f5f5);\
    border-radius:var(--lp-radius,0.5rem);\
    border:1px solid var(--lp-border,#e5e5e5)";

const DEFAULT_INPUT_STYLE: &str = "flex:1;background:transparent;border:none;outline:none;\
    color:var(--lp-text,inherit);font-size:0.875rem";

const DEFAULT_KBD_STYLE: &str = "display:inline-flex;align-items:center;justify-content:center;\
    min-width:1.25rem;height:1.25rem;padding:0 0.375rem;\
    font-family:inherit;font-size:0.6875rem;line-height:1;\
    color:var(--lp-text-muted,#999);\
    background:var(--lp-kbd-bg,rgba(127,127,127,0.12));\
    border:1px solid var(--lp-border,rgba(127,127,127,0.2));\
    border-radius:0.25rem;\
    pointer-events:none;user-select:none";

/// Debounced search input for filtering icons.
///
/// Emits the filter value after `debounce_ms` of inactivity (default 150ms).
/// Programmatic changes to `value` are emitted immediately (no debounce).
///
/// Renders a `<kbd>/</kbd>` shortcut hint when the field is empty (controlled
/// by `shortcut_hint`). The component does not register a `/` listener
/// itself -- consumers wire that up at the appropriate scope (e.g. the picker
/// container or window). Use [`input_ref`] to focus the input from that
/// handler.
///
/// # Example
///
/// ```rust,ignore
/// let (filter, set_filter) = signal(String::new());
/// <IconSearch value=filter on_change=set_filter />
/// ```
#[component]
pub fn IconSearch(
    /// Current filter value (controlled).
    #[prop(into)]
    value: Signal<String>,
    /// Called with the new filter value after debounce.
    on_change: Callback<String>,
    /// Debounce delay in milliseconds.
    #[prop(default = 150)]
    debounce_ms: u64,
    /// Placeholder text.
    #[prop(into, optional)]
    placeholder: Option<TextProp>,
    /// Accessible label announced by screen readers (default: "Search icons").
    #[prop(into, optional)]
    aria_label: Option<TextProp>,
    /// CSS class for the outer container. When set, suppresses the default
    /// inline container styling.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// CSS class for the `<input>` element. When set, suppresses the default
    /// inline input styling.
    #[prop(into, optional)]
    input_class: Option<TextProp>,
    /// CSS class for the trailing `<kbd>` shortcut chip. When set, suppresses
    /// the default inline kbd styling.
    #[prop(into, optional)]
    kbd_class: Option<TextProp>,
    /// CSS class for the clear (X) button wrapper.
    #[prop(into, optional)]
    clear_class: Option<TextProp>,
    /// Search/clear icon size (default: "18").
    #[prop(into, optional)]
    icon_size: Option<TextProp>,
    /// Search/clear icon stroke color (default: muted CSS var).
    #[prop(into, optional)]
    icon_stroke: Option<TextProp>,
    /// Whether to show the clear (X) button.
    #[prop(default = true)]
    show_clear: bool,
    /// Whether to render the trailing `/` shortcut chip when the input is empty.
    #[prop(default = true)]
    shortcut_hint: bool,
    /// Optional `NodeRef` on the underlying `<input>`. Lets parents
    /// programmatically focus the search field (e.g. for a `/` shortcut).
    #[prop(optional)]
    input_ref: Option<NodeRef<leptos::html::Input>>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "Search icons...".into());
    let aria_label = aria_label.unwrap_or_else(|| "Search icons".into());
    let icon_size = icon_size.unwrap_or_else(|| "18".into());
    let icon_stroke = icon_stroke.unwrap_or_else(|| "var(--lp-text-muted,#999)".into());
    let pending_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let input_ref = input_ref.unwrap_or_default();

    let has_class = class.is_some();
    let has_input_class = input_class.is_some();
    let has_kbd_class = kbd_class.is_some();

    let icon_size_l = icon_size.clone();
    let icon_stroke_l = icon_stroke.clone();
    let icon_size_r = icon_size;
    let icon_stroke_r = icon_stroke;

    let on_input = move |ev: leptos::ev::Event| {
        let new_value = event_target_value(&ev);
        if let Some(handle) = pending_handle.get_value() {
            handle.clear();
        }
        let handle = set_timeout_with_handle(
            move || on_change.run(new_value),
            std::time::Duration::from_millis(debounce_ms),
        )
        .ok();
        pending_handle.set_value(handle);
    };

    let clear = move |_| {
        if let Some(handle) = pending_handle.get_value() {
            handle.clear();
        }
        on_change.run(String::new());
    };

    view! {
        <style>
            "input.lp-search::-webkit-search-cancel-button,\
             input.lp-search::-webkit-search-decoration{\
             -webkit-appearance:none;appearance:none;display:none}"
        </style>
        <div class=move || class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
             style=move || if has_class { "" } else { DEFAULT_CONTAINER_STYLE }>
            <Icon glyph=LucideGlyph::Search
                  size=move || icon_size_l.get()
                  stroke=move || icon_stroke_l.get() />
            <input type="search"
                   role="searchbox"
                   node_ref=input_ref
                   class=move || {
                       let extra = input_class.as_ref().map(|c| c.get().to_string()).unwrap_or_default();
                       format!("lp-search {extra}")
                   }
                   style=move || if has_input_class { "" } else { DEFAULT_INPUT_STYLE }
                   prop:placeholder=move || placeholder.get()
                   prop:value=move || value.get()
                   aria-label=move || aria_label.get()
                   on:input=on_input
            />
            {move || {
                let empty = value.get().is_empty();
                if !empty && show_clear {
                    let has_clear_class = clear_class.is_some();
                    let clear_class = clear_class.clone();
                    let icon_size = icon_size_r.clone();
                    let icon_stroke = icon_stroke_r.clone();
                    Some(view! {
                        <span class=move || clear_class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
                              style=move || if has_clear_class { "" } else { "cursor:pointer;display:flex" }
                              role="button"
                              aria-label="Clear search"
                              on:click=clear>
                            <Icon glyph=LucideGlyph::X
                                  size=move || icon_size.get()
                                  stroke=move || icon_stroke.get() />
                        </span>
                    }.into_any())
                } else if empty && shortcut_hint {
                    let kbd_class = kbd_class.clone();
                    Some(view! {
                        <kbd class=move || kbd_class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
                             style=move || if has_kbd_class { "" } else { DEFAULT_KBD_STYLE }
                             aria-label="Press slash to focus search"
                             title="Press / to focus">
                            "/"
                        </kbd>
                    }.into_any())
                } else {
                    None
                }
            }}
        </div>
    }
}
