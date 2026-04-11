use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::LucideGlyph;

use crate::IconPicker;

/// Popover that shows an [`IconPicker`] when a trigger element is clicked.
///
/// # Example
///
/// ```rust,ignore
/// let (icon, set_icon) = signal(None::<LucideGlyph>);
/// <IconPickerPopover
///     selected=icon
///     on_select=Callback::new(move |g| set_icon.set(Some(g)))
/// >
///     <button>"Choose icon"</button>
/// </IconPickerPopover>
/// ```
#[component]
pub fn IconPickerPopover(
    /// Currently selected icon.
    #[prop(into)]
    selected: Signal<Option<LucideGlyph>>,
    /// Called when an icon is selected.
    on_select: Callback<LucideGlyph>,
    /// Trigger element (the clickable thing that opens the popover).
    children: Children,
    /// CSS class for the popover panel.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// Whether selecting an icon closes the popover.
    #[prop(default = true)]
    close_on_select: bool,
    /// Popover width. Default: "480px".
    #[prop(into, optional)]
    width: Option<TextProp>,
    /// Popover height. Default: "400px".
    #[prop(into, optional)]
    height: Option<TextProp>,
) -> impl IntoView {
    let width = width.unwrap_or_else(|| "480px".into());
    let height = height.unwrap_or_else(|| "400px".into());
    let (open, set_open) = signal(false);

    let toggle_open = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        set_open.set(!open.get());
    };

    // Close on Escape
    let keydown_handle = window_event_listener(
        leptos::ev::keydown,
        move |ev: leptos::ev::KeyboardEvent| {
            if ev.key() == "Escape" {
                set_open.set(false);
            }
        },
    );

    // Close on outside click
    let click_handle = window_event_listener(leptos::ev::click, move |_| {
        if open.get_untracked() {
            set_open.set(false);
        }
    });

    on_cleanup(move || {
        keydown_handle.remove();
        click_handle.remove();
    });

    let wrapped_on_select = Callback::new(move |glyph: LucideGlyph| {
        on_select.run(glyph);
        if close_on_select {
            set_open.set(false);
        }
    });

    let class_stored = StoredValue::new(class);
    let height_stored = StoredValue::new(height.clone());

    let width_stored = StoredValue::new(width);
    let panel_style = move || {
        format!(
            "position:absolute;top:100%;left:0;z-index:50;\
             width:{};height:{};\
             margin-top:0.25rem;\
             box-shadow:0 4px 6px -1px rgba(0,0,0,0.1),0 2px 4px -2px rgba(0,0,0,0.1)",
            width_stored.with_value(|w| w.get()),
            height_stored.with_value(|h| h.get())
        )
    };

    view! {
        <div style="position:relative;display:inline-block">
            <div on:click=toggle_open style="cursor:pointer">
                {children()}
            </div>
            {move || open.get().then(|| view! {
                <div style=panel_style
                     class=move || class_stored.with_value(|c| c.as_ref().map(|c| c.get().to_string()).unwrap_or_default())
                     on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()>
                    <IconPicker
                        selected=selected
                        on_select=wrapped_on_select
                        max_height=move || height_stored.with_value(|h| h.get())
                    />
                </div>
            })}
        </div>
    }
}
