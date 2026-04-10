use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::{Icon, LucideGlyph};

/// Debounced search input for filtering icons.
///
/// Emits the filter value after `debounce_ms` of inactivity (default 150ms).
/// Programmatic changes to `value` are emitted immediately (no debounce).
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
    on_change: WriteSignal<String>,
    /// Debounce delay in milliseconds.
    #[prop(default = 150)]
    debounce_ms: u64,
    /// Placeholder text.
    #[prop(into, optional)]
    placeholder: Option<TextProp>,
    /// CSS class for the outer container div.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// Whether to show the clear (X) button.
    #[prop(default = true)]
    show_clear: bool,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "Search icons...".into());

    let on_input = move |ev: leptos::ev::Event| {
        let new_value = event_target_value(&ev);
        on_change.set(new_value.clone());
        let timeout_value = new_value;
        set_timeout(
            move || {
                if value.get_untracked() == timeout_value {
                    on_change.set(value.get_untracked());
                }
            },
            std::time::Duration::from_millis(debounce_ms),
        );
    };

    let clear = move |_| on_change.set(String::new());

    let container_style = "display:flex;align-items:center;gap:0.5rem;\
        padding:0.5rem 1rem;\
        background:var(--lp-bg,#f5f5f5);\
        border-radius:var(--lp-radius,0.5rem);\
        border:1px solid var(--lp-border,#e5e5e5)";

    let input_style = "flex:1;background:transparent;border:none;outline:none;\
        color:var(--lp-text,inherit);font-size:0.875rem";

    view! {
        <div class=move || class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
             style=container_style>
            <Icon glyph=LucideGlyph::Search size="18"
                  stroke=move || "var(--lp-text-muted,#999)".to_string() />
            <input type="text"
                   style=input_style
                   prop:placeholder=move || placeholder.get()
                   prop:value=move || value.get()
                   on:input=on_input
            />
            {show_clear.then(|| view! {
                <span style="cursor:pointer;display:flex" on:click=clear>
                    <Icon glyph=LucideGlyph::X size="16"
                          stroke=move || "var(--lp-text-muted,#999)".to_string() />
                </span>
            })}
        </div>
    }
}
