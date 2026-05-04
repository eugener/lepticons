use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::LucideGlyph;

use crate::theme;

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
        <div class=move || class.with_value(theme::class_str)
             style=move || class.with_value(|c| theme::style_str(c, container_style))>
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
                        if !has_item_class {
                            return String::new();
                        }
                        // Active row uses item_active_class when supplied,
                        // otherwise falls back to item_class so it never
                        // renders unstyled.
                        if is_active() {
                            let active = item_active_class.with_value(theme::class_str);
                            if !active.is_empty() {
                                return active;
                            }
                        }
                        item_class.with_value(theme::class_str)
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
