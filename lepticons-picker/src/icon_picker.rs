use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos::wasm_bindgen::JsCast;
use lepticons::LucideGlyph;

use crate::{CategoryFilter, IconGrid, IconSearch};

/// Inline icon picker with search, category filter, and selectable grid.
///
/// Drop this into a form, settings panel, or editor to let users pick an icon.
///
/// Pressing `/` while focus is anywhere inside the picker (other than the
/// search input itself) jumps focus to the search field.
///
/// # Example
///
/// ```rust,ignore
/// let (icon, set_icon) = signal(None::<LucideGlyph>);
/// <IconPicker
///     selected=icon
///     on_select=Callback::new(move |g| set_icon.set(Some(g)))
/// />
/// ```
#[component]
pub fn IconPicker(
    /// Currently selected icon.
    #[prop(into)]
    selected: Signal<Option<LucideGlyph>>,
    /// Called when an icon is selected.
    on_select: Callback<LucideGlyph>,
    /// Whether to show the category sidebar.
    #[prop(default = true)]
    show_categories: bool,
    /// Whether to show the search bar.
    #[prop(default = true)]
    show_search: bool,
    /// CSS class for the outer container.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// Maximum height (enables scroll). Default: "400px".
    #[prop(into, optional)]
    max_height: Option<TextProp>,
) -> impl IntoView {
    let max_height = max_height.unwrap_or_else(|| "400px".into());
    let (filter, set_filter) = signal(String::new());
    let search_input_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    let on_category = Callback::new(move |cat: String| {
        set_filter.set(cat);
    });

    let container_style = move || {
        format!(
            "display:flex;flex-direction:column;\
             max-height:{};\
             background:var(--lp-bg,#fff);\
             border:1px solid var(--lp-border,#e5e5e5);\
             border-radius:var(--lp-radius,0.5rem);\
             overflow:hidden",
            max_height.get()
        )
    };

    // `/` focuses the search input when focus is anywhere in the picker
    // (except already inside an input/textarea).
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() != "/" || ev.ctrl_key() || ev.meta_key() || ev.alt_key() {
            return;
        }
        if let Some(target) = ev.target() {
            if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
                let tag = el.tag_name();
                if tag.eq_ignore_ascii_case("input") || tag.eq_ignore_ascii_case("textarea") {
                    return;
                }
            }
        }
        if let Some(input) = search_input_ref.get() {
            ev.prevent_default();
            let _ = input.focus();
        }
    };

    view! {
        <div class=move || class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
             style=container_style
             on:keydown=on_keydown>
            {show_search.then(|| view! {
                <div style="padding:0.5rem">
                    <IconSearch
                        value=filter
                        on_change=Callback::new(move |v| set_filter.set(v))
                        input_ref=search_input_ref
                    />
                </div>
            })}
            <div style="display:flex;flex:1;overflow:hidden">
                {show_categories.then(|| view! {
                    <div style="width:10rem;flex-shrink:0;overflow-y:auto;\
                                padding:0.5rem;\
                                border-right:1px solid var(--lp-border,#e5e5e5)">
                        <CategoryFilter
                            on_select=on_category
                            active=filter
                        />
                    </div>
                })}
                <div style="flex:1;overflow-y:auto;padding:0.5rem">
                    <IconGrid
                        filter=filter
                        selected=selected
                        on_select=on_select
                    />
                </div>
            </div>
        </div>
    }
}
