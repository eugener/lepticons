use convert_case::{Case, Casing};
use leptos::ev::*;
use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, JsValue};
use strum::IntoEnumIterator;
use web_sys::js_sys;
use web_sys::wasm_bindgen;

use leptos_router::hooks::{use_params_map, use_query_map};
use lepticons::LucideGlyph;
use lepticons::*;
use lepticons_animate::{AnimationStyles, DrawIcon};
use lepticons_picker::{mru, CategoryFilter, IconGrid, IconSearch, MruStrip};
use crate::components::*;
use crate::menu::*;

const DEFAULT_STROKE_WIDTH: f64 = 2.0;
const DEFAULT_SIZE: f64 = 24.0;

/// localStorage key for the icons-page MRU strip. Distinct from the picker's
/// default key so the two MRU lists evolve independently.
const ICONS_MRU_KEY: &str = "lepticons-icons-mru";

/// Number of related icons to surface in the detail drawer. Matched by tag
/// overlap with the active glyph.
const RELATED_LIMIT: usize = 8;

#[component]
pub fn IconsView() -> impl IntoView {
    let (icon_filter, set_icon_filter) = signal("".to_string());
    let (selected_icon, set_selected_icon) = signal(None::<LucideGlyph>);
    let search_input_ref: NodeRef<leptos::html::Input> = NodeRef::new();
    let clear_filter = move |_| set_icon_filter.set("".to_string());

    // Customizer state (None = use currentColor, adapts to theme)
    let (icon_color, set_icon_color) = signal(None::<String>);
    let (icon_stroke_width, set_icon_stroke_width) = signal(DEFAULT_STROKE_WIDTH);
    let (icon_size, set_icon_size) = signal(DEFAULT_SIZE);
    let (absolute_stroke, set_absolute_stroke) = signal(false);

    let mru_signal: RwSignal<Vec<LucideGlyph>> = RwSignal::new(mru::load(ICONS_MRU_KEY));
    let (help_open, set_help_open) = signal(false);

    // Hydrate state from URL on mount.
    let query = use_query_map();
    {
        let q = query.get_untracked();
        if let Some(v) = q.get("q") {
            set_icon_filter.set(v);
        }
        if let Some(v) = q.get("color") {
            set_icon_color.set(Some(v));
        }
        if let Some(v) = q.get("sw").and_then(|s| s.parse::<f64>().ok()) {
            set_icon_stroke_width.set(v);
        }
        if let Some(v) = q.get("size").and_then(|s| s.parse::<f64>().ok()) {
            set_icon_size.set(v);
        }
        if q.get("abs").as_deref() == Some("1") {
            set_absolute_stroke.set(true);
        }
    }

    // Persist customizer + filter to URL via history.replaceState. No router
    // navigate so we don't trigger remount on every keystroke.
    Effect::new(move |_| {
        let f = icon_filter.get();
        let c = icon_color.get();
        let sw = icon_stroke_width.get();
        let sz = icon_size.get();
        let abs = absolute_stroke.get();
        let mut params: Vec<String> = Vec::new();
        if !f.is_empty() {
            params.push(format!("q={}", js_sys::encode_uri_component(&f)));
        }
        if let Some(c) = c {
            params.push(format!("color={}", js_sys::encode_uri_component(&c)));
        }
        if (sw - DEFAULT_STROKE_WIDTH).abs() > f64::EPSILON {
            params.push(format!("sw={}", sw));
        }
        if (sz - DEFAULT_SIZE).abs() > f64::EPSILON {
            params.push(format!("size={}", sz));
        }
        if abs {
            params.push("abs=1".to_string());
        }
        let qs = if params.is_empty() {
            "/".to_string()
        } else {
            format!("/?{}", params.join("&"))
        };
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                let _ = history.replace_state_with_url(&JsValue::null(), "", Some(&qs));
            }
        }
    });

    // Selecting an icon pushes it to the MRU list and persists to localStorage.
    Effect::new(move |_| {
        if let Some(icon) = selected_icon.get() {
            mru_signal.update(|v| mru::push_into(v, icon));
            mru::save(ICONS_MRU_KEY, &mru_signal.get_untracked());
        }
    });

    // close detail panel when filter changes (IconSearch already debounces)
    Effect::new(move |_| {
        icon_filter.track();
        set_selected_icon.set(None);
    });

    // Window-level shortcuts: Escape closes drawer/help; `/` focuses search;
    // `?` (Shift+/) toggles the keyboard help overlay.
    window_event_listener(keydown, move |ev: KeyboardEvent| {
        let in_input = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            .map(|el| {
                let tag = el.tag_name();
                tag.eq_ignore_ascii_case("input") || tag.eq_ignore_ascii_case("textarea")
            })
            .unwrap_or(false);

        match ev.key().as_str() {
            "Escape" => {
                if help_open.get_untracked() {
                    set_help_open.set(false);
                } else {
                    set_selected_icon.set(None);
                }
            }
            "/" if !in_input && !ev.ctrl_key() && !ev.meta_key() && !ev.alt_key() => {
                if let Some(input_el) = search_input_ref.get() {
                    ev.prevent_default();
                    let _ = input_el.focus();
                }
            }
            "?" if !in_input => {
                ev.prevent_default();
                set_help_open.update(|b| *b = !*b);
            }
            _ => {}
        }
    });

    let icon_count = LucideGlyph::count();
    let drawer_open = move || selected_icon.get().is_some();
    let mru_visible =
        move || icon_filter.get().is_empty() && !mru_signal.with(Vec::is_empty);

    let result_count = Memo::new(move |_| LucideGlyph::find(&icon_filter.get()).len());

    view! {
        <div class="flex flex-row">
            // ----- left sidebar -----
            <div class="w-64 flex-none bg-secondary h-screen overflow-y-auto">

                <StickyTop class="px-10 bg-gradient-to-b from-95% from-secondary to-100% to-transparent">
                    <div class="flex flex-col items-center gap-0 cursor-pointer">
                        <img src="lepticons.png" class="pt-5 w-48"/>
                        <div class="flex flex-row w-full justify-between pb-2 text-primary text-xs items-center">
                            <span on:click=clear_filter>
                                {format!("{} icons", icon_count)}
                            </span>
                            <VersionLink/>
                        </div>
                   </div>
                   <hr/>
                </StickyTop>

                <Customizer
                    color=icon_color
                    set_color=set_icon_color
                    stroke_width=icon_stroke_width
                    set_stroke_width=set_icon_stroke_width
                    size=icon_size
                    set_size=set_icon_size
                    absolute_stroke=absolute_stroke
                    set_absolute_stroke=set_absolute_stroke
                />

                <hr class="mx-10"/>

                <CategoryFilter
                    on_select=Callback::new(move |cat: String| set_icon_filter.set(cat))
                    item_class="flex flex-row justify-between text-sm text-primary/70 cursor-pointer"
                    item_active_class="flex flex-row justify-between text-sm text-highlight font-medium cursor-pointer"
                    class="px-10 pt-5 flex flex-col gap-2"
                />

            </div>

            // ----- main column (search + grid) -----
            <div class="px-10 mt-5 flex flex-col flex-auto h-screen overflow-y-auto overflow-x-hidden transition-all duration-200">
                <StickyTop class="bg-gradient-to-b from-85% from-background to-100% to-transparent">
                    <MainMenu class="justify-end text-primary"/>
                    <Hero/>
                    <div class="flex flex-row items-center gap-3 my-6">
                        <IconSearch
                            value=icon_filter
                            on_change=Callback::new(move |v| set_icon_filter.set(v))
                            input_ref=search_input_ref
                            class="flex flex-row items-center gap-2 w-full p-2 px-4 bg-secondary rounded-lg border border-transparent focus-within:border-highlight/80 transition-colors"
                            input_class="flex-auto p-2 bg-transparent focus:outline-none text-primary"
                            kbd_class="inline-flex items-center justify-center min-w-5 h-5 px-1.5 text-[0.6875rem] leading-none font-mono text-primary/60 bg-primary/10 border border-primary/15 rounded select-none"
                            clear_class="cursor-pointer flex"
                            icon_size="24"
                            icon_stroke="currentColor"
                        />
                        <button
                            class="flex-none w-9 h-9 rounded-lg
                                   border border-primary/15 bg-secondary text-primary/65
                                   hover:border-highlight/60 hover:text-primary
                                   flex items-center justify-center
                                   text-sm font-semibold transition-colors"
                            title="Keyboard shortcuts (?)"
                            aria-label="Open keyboard shortcuts"
                            on:click=move |_| set_help_open.update(|b| *b = !*b)
                        >
                            "?"
                        </button>
                    </div>
                </StickyTop>

                // aria-live announcer for screen readers when the filter changes
                <div class="sr-only" aria-live="polite" aria-atomic="true">
                    {move || {
                        let n = result_count.get();
                        let f = icon_filter.get();
                        if f.is_empty() {
                            format!("{} icons available.", n)
                        } else {
                            format!("{} icons match \"{}\".", n, f)
                        }
                    }}
                </div>

                {move || mru_visible().then(|| view! {
                    <MruStrip
                        mru=mru_signal
                        on_select=Callback::new(move |g| set_selected_icon.set(Some(g)))
                        class="flex flex-row items-center gap-3 mb-4"
                        header_class="text-[0.6875rem] uppercase tracking-wider text-primary/50 font-medium flex-none"
                        item_class=ICON_STYLE
                        icon_size=Signal::derive(move || format!("{}", icon_size.get() as u32))
                        icon_stroke=Signal::derive(move || icon_color.get().unwrap_or_else(|| "currentColor".to_string()))
                        icon_stroke_width=Signal::derive(move || {
                            let sw = icon_stroke_width.get();
                            if absolute_stroke.get() {
                                format!("{:.2}", sw * 24.0 / icon_size.get())
                            } else {
                                format!("{:.2}", sw)
                            }
                        })
                    />
                })}

                <IconGrid
                    filter=icon_filter
                    selected=selected_icon
                    on_select=Callback::new(move |g| set_selected_icon.set(Some(g)))
                    class="flex flex-row flex-wrap gap-2 pb-12"
                    cell_class=ICON_STYLE
                    cell_selected_class=ICON_STYLE_SELECTED
                    tooltip_class=TOOLTIP_STYLE
                    icon_size=move || format!("{}", icon_size.get() as u32)
                    icon_stroke=move || icon_color.get().unwrap_or_else(|| "currentColor".to_string())
                    icon_stroke_width=move || {
                        let sw = icon_stroke_width.get();
                        if absolute_stroke.get() {
                            format!("{:.2}", sw * 24.0 / icon_size.get())
                        } else {
                            format!("{:.2}", sw)
                        }
                    }
                />
            </div>

            // ----- right detail drawer (slides in when an icon is selected) -----
            <div class=move || {
                let open = drawer_open();
                if open {
                    "flex-none w-[26rem] h-screen overflow-y-auto bg-secondary border-l border-primary/15 transition-all duration-200"
                } else {
                    "flex-none w-0 h-screen overflow-hidden border-l border-primary/15 transition-all duration-200"
                }
            }>
                <IconDetailDrawer
                    selected_icon=selected_icon
                    set_selected_icon=set_selected_icon
                />
            </div>
        </div>
        <AnimationStyles/>
        {move || help_open.get().then(|| view! {
            <KeyboardHelp on_close=Callback::new(move |_| set_help_open.set(false)) />
        })}
    }
}

/// Permalink view for a single icon at `/icons/:name`.
/// Supports PascalCase (`/icons/ArrowRight`) and kebab-case (`/icons/arrow-right`).
#[component]
pub fn IconPermalinkView() -> impl IntoView {
    let params = use_params_map();

    let icon = move || {
        let p = params.get();
        let name = p.get("name")?;
        LucideGlyph::by_name(&name)
            .or_else(|| LucideGlyph::by_name(&name.to_case(Case::UpperCamel)))
    };

    view! {
        {move || match icon() {
            Some(glyph) => {
                let name = display_name(&glyph);
                let tags: Vec<&str> = glyph.tags().collect();
                let categories: Vec<&str> = glyph.categories().collect();
                let component_name = glyph.name();
                let first_cat = glyph.categories().next().map(|c| c.to_case(Case::Snake));

                view! {
                    <div class="flex flex-col items-center justify-center min-h-screen p-10 gap-6">
                        <div class="text-primary">
                            <DrawIcon glyph=glyph size="128" stroke_width="1.5" duration_ms=600 />
                        </div>
                        <h1 class="text-3xl font-medium text-primary">{name}</h1>

                        {if !tags.is_empty() {
                            let tag_str = tags.join(" \u{2022} ");
                            Some(view! { <p class="text-sm text-primary/60">{tag_str}</p> })
                        } else {
                            None
                        }}

                        {if !categories.is_empty() {
                            Some(view! {
                                <div class="flex flex-row gap-2 flex-wrap justify-center">
                                    {categories.iter().map(|cat| {
                                        let cat = cat.to_case(Case::Title);
                                        view! {
                                            <span class="px-3 py-1 text-xs rounded-full border border-primary/20 text-primary/70">{cat}</span>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            })
                        } else {
                            None
                        }}

                        // Usage snippet
                        <div class="mt-4 bg-secondary rounded-lg p-4 text-sm font-mono text-primary/80 max-w-lg w-full">
                            {first_cat.map(|cat| view! {
                                <p class="text-primary/50 text-xs mb-2">
                                    {format!("// lepticons = {{ version = \"0.10\", default-features = false, features = [\"{}\"] }}", cat)}
                                </p>
                            })}
                            <p>{format!("<Icon glyph=LucideGlyph::{} />", component_name)}</p>
                        </div>

                        <a href="/" class="mt-4 text-sm text-highlight hover:underline">"Browse all icons"</a>
                    </div>
                }.into_any()
            }
            None => view! {
                <div class="flex flex-col items-center justify-center h-screen">
                    <div class="text-4xl font-medium text-primary">"404"</div>
                    <div class="text-2xl font-medium text-primary/70">"Icon not found"</div>
                    <a href="/" class="mt-4 text-sm text-highlight hover:underline">"Browse all icons"</a>
                </div>
            }.into_any(),
        }}
    }
}

#[component]
fn Customizer(
    color: ReadSignal<Option<String>>,
    set_color: WriteSignal<Option<String>>,
    stroke_width: ReadSignal<f64>,
    set_stroke_width: WriteSignal<f64>,
    size: ReadSignal<f64>,
    set_size: WriteSignal<f64>,
    absolute_stroke: ReadSignal<bool>,
    set_absolute_stroke: WriteSignal<bool>,
) -> impl IntoView {
    let reset_defaults = move |_| {
        set_color.set(None);
        set_stroke_width.set(DEFAULT_STROKE_WIDTH);
        set_size.set(DEFAULT_SIZE);
        set_absolute_stroke.set(false);
    };

    view! {
        <div class="mx-6 mt-5 p-4 flex flex-col gap-4 bg-primary/5 rounded-lg border border-primary/10">
            <div class="flex flex-row items-center justify-between">
                <span class="text-sm font-semibold text-primary">"Customizer"</span>
                <button
                    class="p-1 text-primary/50 hover:text-primary rounded"
                    title="Reset to defaults"
                    on:click=reset_defaults
                >
                    <Icon glyph=LucideGlyph::RotateCcw size="14" />
                </button>
            </div>

            // Color
            <div class="flex flex-col gap-1">
                <label class="text-xs text-primary/60">"Color"</label>
                <div class="flex flex-row items-center gap-2">
                    {
                        let dark_mode = use_context::<crate::components::DarkMode>().map(|d| d.read);
                        let resolved_color = move || {
                            // Touch dark_mode to re-run when theme changes
                            if let Some(dm) = dark_mode { dm.get(); }
                            color.get().unwrap_or_else(current_text_color_hex)
                        };
                        let resolved_color2 = resolved_color;
                        view! {
                            <input type="color"
                                class="w-8 h-8 rounded cursor-pointer border-none bg-transparent p-0"
                                prop:value=resolved_color
                                on:input=move |ev| set_color.set(Some(event_target_value(&ev)))
                            />
                            <span class="text-xs text-primary/70 font-mono">
                                {resolved_color2}
                            </span>
                        }
                    }
                </div>
            </div>

            // Stroke width
            <div class="flex flex-col gap-1">
                <div class="flex flex-row justify-between">
                    <label class="text-xs text-primary/60">"Stroke width"</label>
                    <span class="text-xs text-primary/70">{move || format!("{}px", stroke_width.get())}</span>
                </div>
                <input type="range"
                    class="w-full"
                    min="0.5" max="3" step="0.25"
                    prop:value=move || stroke_width.get().to_string()
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                            set_stroke_width.set(v);
                        }
                    }
                />
            </div>

            // Size
            <div class="flex flex-col gap-1">
                <div class="flex flex-row justify-between">
                    <label class="text-xs text-primary/60">"Size"</label>
                    <span class="text-xs text-primary/70">{move || format!("{}px", size.get() as u32)}</span>
                </div>
                <input type="range"
                    class="w-full"
                    min="14" max="48" step="1"
                    prop:value=move || (size.get() as u32).to_string()
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                            set_size.set(v);
                        }
                    }
                />
            </div>

            // Absolute stroke width toggle
            <div class="flex flex-row items-center justify-between">
                <label class="text-xs text-primary/60 leading-tight">"Absolute stroke"<br/>"width"</label>
                <button
                    class=move || {
                        if absolute_stroke.get() {
                            "w-10 h-5 rounded-full bg-highlight relative transition-colors"
                        } else {
                            "w-10 h-5 rounded-full bg-primary/20 relative transition-colors"
                        }
                    }
                    on:click=move |_| set_absolute_stroke.set(!absolute_stroke.get())
                >
                    <span class=move || {
                        if absolute_stroke.get() {
                            "absolute top-0.5 right-0.5 w-4 h-4 rounded-full bg-white transition-all"
                        } else {
                            "absolute top-0.5 left-0.5 w-4 h-4 rounded-full bg-white transition-all"
                        }
                    }/>
                </button>
            </div>
        </div>
    }
}

// Style strings forwarded to <IconGrid> via cell_class / cell_selected_class /
// tooltip_class. Kept as `pub(crate)`-style constants so the picker conversion
// preserves byte-identical Tailwind output for cells and tooltips.
const ICON_STYLE: &str = "relative group p-2 bg-secondary rounded-lg hover:bg-primary/20 border border-transparent cursor-pointer";
const ICON_STYLE_SELECTED: &str = "relative group p-2 bg-primary/10 rounded-lg border border-highlight/80 cursor-pointer";
const TOOLTIP_STYLE: &str = "absolute left-1/2 -translate-x-1/2 -bottom-4 z-10 opacity-0 transition-opacity group-hover:opacity-100 py-0.5 px-1 text-[0.5rem] font-light text-white bg-highlight/90 border border-highlight/90 rounded whitespace-nowrap";

/// Formats an enum name like "BatteryCharging" to kebab-case "battery-charging".
fn display_name(icon: &LucideGlyph) -> String {
    icon.kebab_name()
}

// ----------------------------------------------------------------------------
// Hero, PerfPulse, MruStrip, KeyboardHelp
// ----------------------------------------------------------------------------

#[component]
fn Hero() -> impl IntoView {
    view! {
        <div class="pt-2 pb-1">
            <div class="text-[0.6875rem] uppercase tracking-wider text-highlight font-medium">
                "Open-source icons for Leptos"
            </div>
            <h1 class="text-2xl font-semibold text-primary leading-tight mt-1">
                "Search, customize, copy."
            </h1>
        </div>
    }
}

#[component]
fn KeyboardHelp(on_close: Callback<()>) -> impl IntoView {
    let rows: [(&str, &str); 8] = [
        ("/", "Focus search"),
        ("?", "Toggle this help"),
        ("Arrow keys", "Move focus across the grid"),
        ("Home / End", "Jump to first / last icon"),
        ("Page Up / Down", "Move five rows at a time"),
        ("Enter", "Open the focused icon's detail"),
        ("Esc", "Close the detail drawer or this help"),
        ("Click", "Select an icon (or open it from Recent)"),
    ];
    view! {
        <div
            class="fixed inset-0 z-[60] bg-primary/40 flex items-center justify-center p-6"
            role="dialog"
            aria-modal="true"
            aria-label="Keyboard shortcuts"
            on:click=move |_| on_close.run(())
        >
            <div
                class="bg-background border border-primary/15 rounded-lg shadow-xl
                       w-full max-w-md p-6"
                on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()
            >
                <div class="flex flex-row items-start justify-between mb-4">
                    <div>
                        <div class="text-[0.6875rem] uppercase tracking-wider text-highlight font-medium">
                            "Keyboard"
                        </div>
                        <h2 class="text-lg font-semibold text-primary mt-0.5">
                            "Shortcuts"
                        </h2>
                    </div>
                    <button
                        class="flex-none p-1 text-primary/50 hover:text-primary"
                        aria-label="Close shortcuts"
                        on:click=move |_| on_close.run(())
                    >
                        <Icon glyph=LucideGlyph::X size="18"/>
                    </button>
                </div>
                <div class="flex flex-col gap-2">
                    {rows.iter().map(|(keys, label)| view! {
                        <div class="flex flex-row items-center justify-between gap-3 py-1
                                    border-b border-primary/5 last:border-0">
                            <span class="text-sm text-primary/70">{*label}</span>
                            <kbd class="px-2 py-1 text-[0.6875rem] font-mono text-primary/70
                                        bg-primary/5 border border-primary/15 rounded">
                                {*keys}
                            </kbd>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
                <p class="mt-4 text-xs text-primary/45">
                    "Tip: shortcuts are inactive while you're typing in an input."
                </p>
            </div>
        </div>
    }
}

/// Returns up to `RELATED_LIMIT` icons that share at least one tag with `icon`.
fn related_icons(icon: LucideGlyph) -> Vec<LucideGlyph> {
    let tags: Vec<&str> = icon.tags().collect();
    if tags.is_empty() {
        return Vec::new();
    }
    LucideGlyph::iter()
        .filter(|g| *g != icon)
        .filter(|g| g.tags().any(|t| tags.contains(&t)))
        .take(RELATED_LIMIT)
        .collect()
}

#[component]
fn IconDetailDrawer(
    selected_icon: ReadSignal<Option<LucideGlyph>>,
    set_selected_icon: WriteSignal<Option<LucideGlyph>>,
) -> impl IntoView {
    let dismiss = move |_| set_selected_icon.set(None);

    let (svg_menu_open, set_svg_menu_open) = signal(false);
    let (jsx_menu_open, set_jsx_menu_open) = signal(false);
    let (svg_copied, set_svg_copied) = signal(false);
    let (jsx_copied, set_jsx_copied) = signal(false);
    let (anim_type, set_anim_type) = signal(0usize);
    let (replay_key, set_replay_key) = signal(0u32);
    let (draw_duration, set_draw_duration) = signal(600u32);
    let (draw_delay, set_draw_delay) = signal(0u32);

    const ANIM_TYPES: [(&str, &str); 6] = [
        ("None", ""),
        ("Draw-In", ""),
        ("Spin", "lepticons-spin"),
        ("Pulse", "lepticons-pulse"),
        ("Bounce", "lepticons-bounce"),
        ("Ping", "lepticons-ping"),
    ];

    // Reset menus and animation when the selected icon changes.
    Effect::new(move |_| {
        selected_icon.get();
        set_svg_menu_open.set(false);
        set_jsx_menu_open.set(false);
        set_svg_copied.set(false);
        set_jsx_copied.set(false);
        set_anim_type.set(0);
        set_replay_key.update(|k| *k += 1);
    });

    view! {
        {move || selected_icon.get().map(|icon| {
            let name = display_name(&icon);
            let tags: Vec<&str> = icon.tags().collect();
            let categories: Vec<&str> = icon.categories().collect();
            let related = related_icons(icon);
            let svg_content = icon.svg();
            let full_svg = format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\">{}</svg>",
                svg_content
            );

            let icon_name = name.clone();
            let component_name = icon.name();
            let kebab_name = name.clone();
            let first_feature = StoredValue::new(
                icon.categories().next().map(|c| c.to_case(Case::Snake))
            );

            view! {
                <div class="flex flex-col gap-4 p-5 min-h-screen"
                     on:click=move |_| { set_svg_menu_open.set(false); set_jsx_menu_open.set(false); }>

                    // header: kicker + name + close
                    <div class="flex flex-row items-start justify-between gap-3">
                        <div class="min-w-0">
                            <div class="text-[0.6875rem] uppercase tracking-wider text-highlight font-medium">
                                "Icon"
                            </div>
                            <h2 class="text-lg font-semibold text-primary truncate">
                                {name.clone()}
                            </h2>
                        </div>
                        <button
                            class="flex-none p-1 text-primary/50 hover:text-primary"
                            aria-label="Close detail drawer"
                            on:click=dismiss
                        >
                            <Icon glyph=LucideGlyph::X size="18"/>
                        </button>
                    </div>

                    // preview
                    <div class="w-full aspect-square flex items-center justify-center
                                rounded-xl bg-background border border-primary/10"
                         style="background-image: linear-gradient(to right, rgba(128,128,128,0.12) 1px, transparent 1px), linear-gradient(to bottom, rgba(128,128,128,0.12) 1px, transparent 1px); background-size: calc(100% / 24) calc(100% / 24); background-position: 0 0;">
                        <div class=move || {
                            let idx = anim_type.get();
                            if idx >= 2 { format!("text-primary {}", ANIM_TYPES[idx].1) } else { "text-primary".to_string() }
                        }>
                            {move || {
                                replay_key.get();
                                if anim_type.get() == 1 {
                                    let d = draw_duration.get();
                                    let dl = draw_delay.get();
                                    view! { <DrawIcon glyph=icon size="180" stroke_width="2" duration_ms=d delay_ms=dl /> }.into_any()
                                } else {
                                    view! { <Icon glyph=icon size="180" stroke_width="2" /> }.into_any()
                                }
                            }}
                        </div>
                    </div>

                    {(!tags.is_empty()).then(|| {
                        let tag_str = tags.join(" \u{2022} ");
                        view! { <p class="text-xs text-primary/55 leading-relaxed">{tag_str}</p> }
                    })}

                    {(!categories.is_empty()).then(|| view! {
                        <div class="flex flex-row gap-1.5 flex-wrap">
                            {categories.iter().map(|cat| {
                                let label = cat.to_case(Case::Title);
                                view! {
                                    <span class="px-2 py-0.5 text-[0.6875rem] rounded-full
                                                 border border-primary/15 text-primary/65">
                                        {label}
                                    </span>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    })}

                    // copy actions
                    <div class="flex flex-row gap-2">
                        <div class="relative flex-1">
                            <button
                                class="w-full px-3 py-2 text-sm rounded-lg
                                       border border-primary/15 text-primary/75
                                       hover:bg-primary/10 flex items-center justify-center gap-1.5"
                                aria-haspopup="menu"
                                aria-expanded=move || svg_menu_open.get().to_string()
                                on:click=move |ev: web_sys::MouseEvent| { ev.stop_propagation(); set_svg_menu_open.set(!svg_menu_open.get()); set_jsx_menu_open.set(false); }
                            >
                                <Icon glyph=Signal::derive(move || {
                                    if svg_copied.get() { LucideGlyph::Check } else { LucideGlyph::Copy }
                                }) size="14"/>
                                {move || if svg_copied.get() { "Copied!" } else { "Copy SVG" }}
                                <Icon glyph=LucideGlyph::ChevronDown size="12"/>
                            </button>
                            {move || svg_menu_open.get().then(|| {
                                let svg_for_copy = full_svg.clone();
                                let svg_for_data_url = full_svg.clone();
                                let svg_for_download = full_svg.clone();
                                let svg_for_png = full_svg.clone();
                                let name_for_svg = icon_name.clone();
                                let name_for_png = icon_name.clone();
                                view! {
                                    <div class="absolute top-full left-0 right-0 mt-1
                                                bg-background border border-primary/20 rounded-lg shadow-lg
                                                py-1 z-50" role="menu">
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| copy_and_flash(&svg_for_copy, set_svg_copied, set_svg_menu_open)>
                                            "Copy SVG"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| {
                                                    let data_url = format!("data:image/svg+xml,{}", js_sys::encode_uri_component(&svg_for_data_url));
                                                    copy_and_flash(&data_url, set_svg_copied, set_svg_menu_open);
                                                }>
                                            "Copy Data URL"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| {
                                                    download_blob(&svg_for_download, &format!("{}.svg", name_for_svg), "image/svg+xml");
                                                    set_svg_menu_open.set(false);
                                                }>
                                            "Download SVG"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| {
                                                    download_png(&svg_for_png, &name_for_png);
                                                    set_svg_menu_open.set(false);
                                                }>
                                            "Download PNG"
                                        </button>
                                    </div>
                                }
                            })}
                        </div>
                        <div class="relative flex-1">
                            <button
                                class="w-full px-3 py-2 text-sm rounded-lg
                                       border border-primary/15 text-primary/75
                                       hover:bg-primary/10 flex items-center justify-center gap-1.5"
                                aria-haspopup="menu"
                                aria-expanded=move || jsx_menu_open.get().to_string()
                                on:click=move |ev: web_sys::MouseEvent| { ev.stop_propagation(); set_jsx_menu_open.set(!jsx_menu_open.get()); set_svg_menu_open.set(false); }
                            >
                                {move || if jsx_copied.get() { "Copied!" } else { "Copy JSX" }}
                                <Icon glyph=LucideGlyph::ChevronDown size="12"/>
                            </button>
                            {move || jsx_menu_open.get().then(|| {
                                let comp = component_name;
                                let kebab = kebab_name.clone();
                                let kebab2 = kebab_name.clone();
                                view! {
                                    <div class="absolute top-full right-0 mt-1
                                                bg-background border border-primary/20 rounded-lg shadow-lg
                                                py-1 min-w-[180px] z-50" role="menu">
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| copy_and_flash(&format!("<{} />", comp), set_jsx_copied, set_jsx_menu_open)>
                                            "Copy JSX"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| copy_and_flash(comp, set_jsx_copied, set_jsx_menu_open)>
                                            "Copy Component Name"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| copy_and_flash(&format!("<{} />", kebab), set_jsx_copied, set_jsx_menu_open)>
                                            "Copy Vue"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| copy_and_flash(&format!("<{} />", comp), set_jsx_copied, set_jsx_menu_open)>
                                            "Copy Svelte"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| copy_and_flash(&format!("<lucide-angular name=\"{}\" />", kebab2), set_jsx_copied, set_jsx_menu_open)>
                                            "Copy Angular"
                                        </button>
                                        <button role="menuitem" class="w-full text-left px-3 py-1.5 text-xs text-primary hover:bg-primary/10"
                                                on:click=move |_| {
                                                    let snippet = if let Some(ref feat) = first_feature.get_value() {
                                                        format!(
                                                            "// lepticons = {{ version = \"0.10\", default-features = false, features = [\"{}\"] }}\n\
                                                             use lepticons::{{Icon, LucideGlyph}};\n\n\
                                                             view! {{ <Icon glyph=LucideGlyph::{} /> }}",
                                                            feat, comp
                                                        )
                                                    } else {
                                                        format!("<Icon glyph=LucideGlyph::{} />", comp)
                                                    };
                                                    copy_and_flash(&snippet, set_jsx_copied, set_jsx_menu_open);
                                                }>
                                            "Copy Leptos"
                                        </button>
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    // animation pills
                    <hr class="border-primary/10"/>
                    <div class="flex flex-row flex-wrap gap-1 items-center">
                        <span class="text-[0.6875rem] uppercase tracking-wider text-primary/45 mr-1">
                            "Animate"
                        </span>
                        {ANIM_TYPES.iter().enumerate().map(|(i, (label, _))| {
                            let is_sel = move || anim_type.get() == i;
                            view! {
                                <button
                                    class=move || if is_sel() {
                                        "px-2 py-0.5 text-xs rounded-full border border-highlight/80 bg-highlight/10 text-highlight font-medium cursor-pointer"
                                    } else {
                                        "px-2 py-0.5 text-xs rounded-full border border-primary/20 text-primary/55 hover:bg-primary/10 cursor-pointer"
                                    }
                                    on:click=move |ev: web_sys::MouseEvent| {
                                        ev.stop_propagation();
                                        set_anim_type.set(i);
                                        if i == 1 { set_replay_key.update(|k| *k += 1); }
                                    }
                                >
                                    {*label}
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                        {move || (anim_type.get() == 1).then(|| view! {
                            <button
                                class="px-2 py-0.5 text-xs rounded-full border border-primary/20 text-primary/55 hover:bg-primary/10 flex items-center gap-1 cursor-pointer"
                                on:click=move |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
                                    set_replay_key.update(|k| *k += 1);
                                }
                            >
                                <Icon glyph=LucideGlyph::RotateCcw size="10"/>
                                "Replay"
                            </button>
                        })}
                    </div>

                    // Draw-In controls
                    {move || (anim_type.get() == 1).then(|| view! {
                        <div class="flex flex-col gap-2">
                            <div class="flex flex-row items-center gap-2">
                                <label class="text-xs text-primary/45 w-16 flex-none">"Duration"</label>
                                <input type="range" class="flex-auto" min="100" max="2000" step="100"
                                    prop:value=move || draw_duration.get().to_string()
                                    on:input=move |ev| {
                                        if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                            set_draw_duration.set(v);
                                            set_replay_key.update(|k| *k += 1);
                                        }
                                    }
                                />
                                <span class="text-xs text-primary/55 w-12 text-right">{move || format!("{}ms", draw_duration.get())}</span>
                            </div>
                            <div class="flex flex-row items-center gap-2">
                                <label class="text-xs text-primary/45 w-16 flex-none">"Delay"</label>
                                <input type="range" class="flex-auto" min="0" max="1000" step="100"
                                    prop:value=move || draw_delay.get().to_string()
                                    on:input=move |ev| {
                                        if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                            set_draw_delay.set(v);
                                            set_replay_key.update(|k| *k += 1);
                                        }
                                    }
                                />
                                <span class="text-xs text-primary/55 w-12 text-right">{move || format!("{}ms", draw_delay.get())}</span>
                            </div>
                        </div>
                    })}

                    // related icons
                    {(!related.is_empty()).then(|| view! {
                        <hr class="border-primary/10"/>
                        <div class="flex flex-col gap-2">
                            <div class="text-[0.6875rem] uppercase tracking-wider text-primary/45 font-medium">
                                "Related"
                            </div>
                            <div class="flex flex-row flex-wrap gap-1.5">
                                {related.into_iter().map(|g| {
                                    let label = g.kebab_name();
                                    view! {
                                        <button
                                            class="p-2 rounded-md bg-background border border-primary/10
                                                   text-primary/75
                                                   hover:border-highlight/60 hover:text-primary
                                                   transition-colors"
                                            aria-label=label.clone()
                                            title=label
                                            on:click=move |_| set_selected_icon.set(Some(g))
                                        >
                                            <Icon glyph=g size="20"/>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    })}
                </div>
            }
        })}
    }
}

/// Returns the computed text color of <body> as a hex string (e.g. "#1a1a2e").
fn current_text_color_hex() -> String {
    let fallback = "#000000".to_string();
    let Some(window) = web_sys::window() else { return fallback };
    let Some(document) = window.document() else { return fallback };
    let Some(body) = document.body() else { return fallback };
    let Ok(computed) = window.get_computed_style(&body) else { return fallback };
    let Some(style) = computed else { return fallback };
    let Ok(rgb) = style.get_property_value("color") else { return fallback };
    rgb_to_hex(&rgb).unwrap_or(fallback)
}

/// Converts "rgb(r, g, b)" or "rgba(r, g, b, a)" to "#rrggbb".
fn rgb_to_hex(rgb: &str) -> Option<String> {
    let inner = rgb.trim().strip_prefix("rgb")?.trim_start_matches('a').strip_prefix('(')?.strip_suffix(')')?;
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() < 3 { return None; }
    let r: u8 = parts[0].trim().parse().ok()?;
    let g: u8 = parts[1].trim().parse().ok()?;
    let b: u8 = parts[2].trim().parse().ok()?;
    Some(format!("#{:02x}{:02x}{:02x}", r, g, b))
}

fn copy_to_clipboard(text: &str) {
    if let Some(w) = web_sys::window() {
        let clipboard = w.navigator().clipboard();
        let _ = clipboard.write_text(text);
    }
}

/// Copies text to clipboard, flashes a "copied" signal for 2 seconds, and closes a menu.
fn copy_and_flash(
    text: &str,
    set_copied: WriteSignal<bool>,
    set_menu_open: WriteSignal<bool>,
) {
    copy_to_clipboard(text);
    set_copied.set(true);
    set_menu_open.set(false);
    set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
}

fn download_blob(content: &str, filename: &str, mime: &str) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Some(body) = document.body() else { return };
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(content));
    let opts = web_sys::BlobPropertyBag::new();
    opts.set_type(mime);
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &opts) else { return };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else { return };
    let Ok(el) = document.create_element("a") else { return };
    let anchor: web_sys::HtmlAnchorElement = el.unchecked_into();
    anchor.set_href(&url);
    anchor.set_download(filename);
    let _ = body.append_child(&anchor);
    anchor.click();
    let _ = body.remove_child(&anchor);
    let _ = web_sys::Url::revoke_object_url(&url);
}

fn download_png(svg_str: &str, name: &str) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Some(body) = document.body() else { return };

    let Ok(canvas_el) = document.create_element("canvas") else { return };
    let canvas: web_sys::HtmlCanvasElement = canvas_el.unchecked_into();
    let png_size = 256;
    canvas.set_width(png_size);
    canvas.set_height(png_size);

    let Ok(ctx_val) = canvas.get_context("2d") else { return };
    let Some(ctx_obj) = ctx_val else { return };
    let ctx: web_sys::CanvasRenderingContext2d = ctx_obj.unchecked_into::<web_sys::CanvasRenderingContext2d>();

    let Ok(img) = web_sys::HtmlImageElement::new() else { return };
    let data_url = format!("data:image/svg+xml,{}", js_sys::encode_uri_component(svg_str));

    let filename = format!("{}.png", name);
    let canvas_clone = canvas.clone();
    let img_clone = img.clone();
    let body_clone = body;

    let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
        let Ok(()) = ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &img_clone, 0.0, 0.0, png_size as f64, png_size as f64,
        ) else { return };
        if let Ok(png_url) = canvas_clone.to_data_url_with_type("image/png") {
            let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
            let Ok(el) = document.create_element("a") else { return };
            let anchor: web_sys::HtmlAnchorElement = el.unchecked_into();
            anchor.set_href(&png_url);
            anchor.set_download(&filename);
            let _ = body_clone.append_child(&anchor);
            anchor.click();
            let _ = body_clone.remove_child(&anchor);
        }
    });
    img.set_onload(Some(cb.as_ref().unchecked_ref()));
    img.set_src(&data_url);
}
