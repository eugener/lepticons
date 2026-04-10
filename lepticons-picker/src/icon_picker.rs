use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::LucideGlyph;

use crate::{CategoryFilter, IconGrid, IconSearch};

/// Inline icon picker with search, category filter, and selectable grid.
///
/// Drop this into a form, settings panel, or editor to let users pick an icon.
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

    view! {
        <div class=move || class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
             style=container_style>
            {show_search.then(|| view! {
                <div style="padding:0.5rem">
                    <IconSearch value=Signal::derive(move || filter.get()) on_change=set_filter />
                </div>
            })}
            <div style="display:flex;flex:1;overflow:hidden">
                {show_categories.then(|| view! {
                    <div style="width:10rem;flex-shrink:0;overflow-y:auto;\
                                padding:0.5rem;\
                                border-right:1px solid var(--lp-border,#e5e5e5)">
                        <CategoryFilter
                            on_select=on_category
                            active=Signal::derive(move || filter.get())
                        />
                    </div>
                })}
                <div style="flex:1;overflow-y:auto;padding:0.5rem">
                    <IconGrid
                        filter=Signal::derive(move || filter.get())
                        selected=selected
                        on_select=on_select
                    />
                </div>
            </div>
        </div>
    }
}
