use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::LucideGlyph;

/// Displays all icon categories with their icon counts.
///
/// Clicking a category invokes `on_select` with the category name (Title Case).
///
/// # Example
///
/// ```rust,ignore
/// <CategoryFilter
///     on_select=Callback::new(move |cat: String| set_filter.set(cat))
/// />
/// ```
#[component]
pub fn CategoryFilter(
    /// Called with the category title when clicked.
    on_select: Callback<String>,
    /// Currently active category (used for highlight). Empty string = none.
    #[prop(into, optional)]
    active: Option<Signal<String>>,
    /// CSS class for the container div.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// CSS class for each category row.
    #[prop(into, optional)]
    item_class: Option<TextProp>,
    /// CSS class for the active category row.
    #[prop(into, optional)]
    item_active_class: Option<TextProp>,
) -> impl IntoView {
    let has_class = class.is_some();
    let has_item_class = item_class.is_some();
    let class = StoredValue::new(class);
    let item_class = StoredValue::new(item_class);
    let item_active_class = StoredValue::new(item_active_class);

    let container_style = "display:flex;flex-direction:column;gap:0.25rem";

    let default_row_style = "display:flex;justify-content:space-between;\
        font-size:0.875rem;cursor:pointer;\
        padding:0.125rem 0;\
        color:var(--lp-text,inherit)";

    view! {
        <div class=move || class.with_value(|c| c.as_ref().map(|c| c.get().to_string()).unwrap_or_default())
             style=move || if has_class { "" } else { container_style }>
        {
            move || LucideGlyph::all_categories().iter()
                .filter(|(k, _)| !k.is_empty())
                .map(|(title, count)| {
                    let title_owned = title.clone();
                    let title_for_click = title.clone();
                    let is_active = {
                        let title_check = title.clone();
                        move || active.is_some_and(|a| a.get() == title_check)
                    };

                    let row_class = move || {
                        if has_item_class {
                            // Fall back to item_class when active row has no
                            // dedicated class so it doesn't render unstyled.
                            let active_cls = if is_active() {
                                item_active_class.with_value(|c| c.as_ref().map(|c| c.get().to_string()))
                            } else {
                                None
                            };
                            active_cls.unwrap_or_else(|| {
                                item_class.with_value(|c| c.as_ref().map(|c| c.get().to_string()).unwrap_or_default())
                            })
                        } else {
                            String::new()
                        }
                    };

                    let row_style = move || {
                        if has_item_class { "" } else { default_row_style }
                    };

                    view! {
                        <div class=row_class style=row_style
                             on:click=move |_| on_select.run(title_for_click.clone())>
                            <span style="flex:1">{title_owned}</span>
                            <span style="font-size:0.75rem;opacity:0.5;flex-shrink:0">{*count}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()
        }
        </div>
    }
}
