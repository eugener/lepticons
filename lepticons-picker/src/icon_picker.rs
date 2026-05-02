use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos::wasm_bindgen::JsCast;
use lepticons::{Icon, LucideGlyph};

use crate::copy::IconCopyFormat;
use crate::mru;
use crate::{CategoryFilter, IconGrid, IconSearch};

const DEFAULT_MRU_STORAGE_KEY: &str = "lepticons-picker-mru";

/// Inline icon picker with search, category filter, and selectable grid.
///
/// Drop this into a form, settings panel, or editor to let users pick an icon.
///
/// Pressing `/` while focus is anywhere inside the picker (other than the
/// search input itself) jumps focus to the search field. Recently selected
/// icons are persisted to `localStorage` and surfaced as a "Recently used"
/// strip above the grid (opt out via `mru_enabled=false`).
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
    /// Whether to show the "Recently used" strip and persist selections to
    /// `localStorage`.
    #[prop(default = true)]
    mru_enabled: bool,
    /// `localStorage` key used to persist the MRU list. Override to isolate
    /// the MRU state between multiple picker instances.
    #[prop(default = DEFAULT_MRU_STORAGE_KEY)]
    mru_storage_key: &'static str,
    /// Whether to render the "Copy as" format dropdown and per-cell copy
    /// buttons.
    #[prop(default = true)]
    show_copy: bool,
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

    let mru_signal: RwSignal<Vec<LucideGlyph>> = RwSignal::new(if mru_enabled {
        mru::load(mru_storage_key)
    } else {
        Vec::new()
    });

    let copy_format = RwSignal::new(IconCopyFormat::default());
    let copy_format_signal: Signal<IconCopyFormat> = copy_format.into();

    let wrapped_on_select = Callback::new(move |icon: LucideGlyph| {
        if mru_enabled {
            mru_signal.update(|v| mru::push_into(v, icon));
            mru::save(mru_storage_key, &mru_signal.get_untracked());
        }
        on_select.run(icon);
    });

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
                <div style="padding:0.5rem;display:flex;gap:0.5rem;align-items:center">
                    <div style="flex:1">
                        <IconSearch
                            value=filter
                            on_change=Callback::new(move |v| set_filter.set(v))
                            input_ref=search_input_ref
                        />
                    </div>
                    {show_copy.then(|| view! {
                        <CopyFormatSelect format=copy_format />
                    })}
                </div>
            })}
            {move || (mru_enabled && !mru_signal.with(Vec::is_empty)).then(|| view! {
                <MruStrip mru=mru_signal on_select=wrapped_on_select />
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
                    {if show_copy {
                        view! {
                            <IconGrid
                                filter=filter
                                selected=selected
                                on_select=wrapped_on_select
                                copy_format=copy_format_signal
                            />
                        }.into_any()
                    } else {
                        view! {
                            <IconGrid
                                filter=filter
                                selected=selected
                                on_select=wrapped_on_select
                            />
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn MruStrip(
    mru: RwSignal<Vec<LucideGlyph>>,
    on_select: Callback<LucideGlyph>,
) -> impl IntoView {
    view! {
        <div style="padding:0.25rem 0.75rem 0.5rem;\
                    border-bottom:1px solid var(--lp-border,#e5e5e5)">
            <div style="font-size:0.6875rem;font-weight:500;letter-spacing:0.04em;\
                        text-transform:uppercase;\
                        color:var(--lp-text-muted,#999);\
                        margin-bottom:0.25rem">
                "Recently used"
            </div>
            <div role="list" style="display:flex;flex-wrap:wrap;gap:0.25rem">
                {move || mru.get().into_iter().map(|icon| {
                    let label = icon.kebab_name();
                    view! {
                        <div role="listitem"
                             aria-label=label.clone()
                             title=label
                             style="padding:0.375rem;border-radius:var(--lp-radius,0.5rem);\
                                    background:var(--lp-bg,#f5f5f5);\
                                    cursor:pointer;display:flex"
                             tabindex="0"
                             on:click=move |_| on_select.run(icon)
                             on:keydown=move |ev: web_sys::KeyboardEvent| {
                                 if ev.key() == "Enter" || ev.key() == " " {
                                     ev.prevent_default();
                                     on_select.run(icon);
                                 }
                             }>
                            <Icon glyph=icon size="20" />
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn CopyFormatSelect(format: RwSignal<IconCopyFormat>) -> impl IntoView {
    let on_change = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        if let Some(fmt) = IconCopyFormat::from_id(&value) {
            format.set(fmt);
        }
    };

    let select_style = "padding:0.375rem 0.5rem;\
        font-size:0.75rem;\
        color:var(--lp-text,inherit);\
        background:var(--lp-bg,#f5f5f5);\
        border:1px solid var(--lp-border,#e5e5e5);\
        border-radius:var(--lp-radius,0.375rem);\
        cursor:pointer";

    view! {
        <label style="display:flex;align-items:center;gap:0.375rem;\
                      font-size:0.6875rem;color:var(--lp-text-muted,#999)">
            "Copy as"
            <select
                aria-label="Copy code format"
                style=select_style
                prop:value=move || format.get().id()
                on:change=on_change
            >
                {IconCopyFormat::ALL.iter().copied().map(|fmt| view! {
                    <option value=fmt.id() selected=move || format.get() == fmt>
                        {fmt.label()}
                    </option>
                }).collect::<Vec<_>>()}
            </select>
        </label>
    }
}
