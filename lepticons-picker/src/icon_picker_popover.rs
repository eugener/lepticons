use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos::wasm_bindgen::JsCast;
use lepticons::LucideGlyph;

use crate::IconPicker;

/// Popover that shows an [`IconPicker`] when a trigger element is clicked.
///
/// The popover panel size is taken from the `width` / `height` props on the
/// first open. If the panel has been resized by the user (e.g. via a CSS
/// `resize` rule on the `class` prop), the resized dimensions are captured
/// each time the popover closes and reused as the initial size on the next
/// open.
///
/// Outside-click detection runs on `mousedown` (rather than `click`) so that
/// dragging a CSS resize handle outside the original element does not close
/// the popover.
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
    /// Accessible label for the popover dialog (default: "Choose an icon").
    #[prop(into, optional)]
    aria_label: Option<TextProp>,
) -> impl IntoView {
    let width = width.unwrap_or_else(|| "480px".into());
    let height = height.unwrap_or_else(|| "400px".into());
    let aria_label = aria_label.unwrap_or_else(|| "Choose an icon".into());
    let (open, set_open) = signal(false);

    // The panel size starts from the props but follows any user resize after
    // that. RwSignal so we can capture-on-close and reuse on next open.
    let stored_width: RwSignal<String> = RwSignal::new(width.get().to_string());
    let stored_height: RwSignal<String> = RwSignal::new(height.get().to_string());

    let panel_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let trigger_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Capture the current panel dimensions before unmounting so the next
    // open reuses them. No-op when the panel ref isn't mounted.
    let capture_panel_size = move || {
        if let Some(el) = panel_ref.get() {
            let w = el.offset_width();
            let h = el.offset_height();
            if w > 0 {
                stored_width.set(format!("{}px", w));
            }
            if h > 0 {
                stored_height.set(format!("{}px", h));
            }
        }
    };

    let close = move || {
        capture_panel_size();
        set_open.set(false);
    };

    let toggle_open = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        if open.get() {
            close();
        } else {
            set_open.set(true);
        }
    };

    // Close on Escape.
    let keydown_handle = window_event_listener(
        leptos::ev::keydown,
        move |ev: leptos::ev::KeyboardEvent| {
            if ev.key() == "Escape" && open.get_untracked() {
                close();
            }
        },
    );

    // Close on outside *mousedown* with a target test. Using mousedown (not
    // click) means a drag whose mouseup lands outside the panel -- e.g.
    // dragging the CSS resize handle past the panel edge -- doesn't close
    // the popover.
    let mousedown_handle = window_event_listener(
        leptos::ev::mousedown,
        move |ev: web_sys::MouseEvent| {
            if !open.get_untracked() {
                return;
            }
            let Some(target) = ev.target() else { return; };
            let Ok(node) = target.dyn_into::<web_sys::Node>() else {
                return;
            };
            let inside_panel = panel_ref
                .get()
                .map(|el| el.contains(Some(&node)))
                .unwrap_or(false);
            let inside_trigger = trigger_ref
                .get()
                .map(|el| el.contains(Some(&node)))
                .unwrap_or(false);
            if !inside_panel && !inside_trigger {
                close();
            }
        },
    );

    on_cleanup(move || {
        keydown_handle.remove();
        mousedown_handle.remove();
    });

    let wrapped_on_select = Callback::new(move |glyph: LucideGlyph| {
        on_select.run(glyph);
        if close_on_select {
            close();
        }
    });

    let class_stored = StoredValue::new(class);
    let aria_label_stored = StoredValue::new(aria_label);

    let panel_style = move || {
        format!(
            "position:absolute;top:100%;left:0;z-index:50;\
             width:{};height:{};\
             margin-top:0.25rem;\
             box-shadow:0 4px 6px -1px rgba(0,0,0,0.1),0 2px 4px -2px rgba(0,0,0,0.1)",
            stored_width.get(),
            stored_height.get()
        )
    };

    view! {
        <div style="position:relative;display:inline-block">
            <div node_ref=trigger_ref on:click=toggle_open style="cursor:pointer">
                {children()}
            </div>
            {move || open.get().then(|| view! {
                <div node_ref=panel_ref
                     style=panel_style
                     class=move || class_stored.with_value(|c| c.as_ref().map(|c| c.get().to_string()).unwrap_or_default())
                     role="dialog"
                     aria-modal="true"
                     aria-label=move || aria_label_stored.with_value(|a| a.get().to_string())>
                    <IconPicker
                        selected=selected
                        on_select=wrapped_on_select
                        max_height=move || stored_height.get()
                    />
                </div>
            })}
        </div>
    }
}
