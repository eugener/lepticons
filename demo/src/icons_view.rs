use convert_case::{Case, Casing};
use leptos::ev::*;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::js_sys;
use web_sys::wasm_bindgen;

use leptos_router::hooks::use_params_map;
use lepticons::LucideGlyph;
use lepticons::*;
use lepticons_animate::{AnimationStyles, DrawIcon};
use lepticons_picker::{CategoryFilter, IconGrid};
use crate::components::*;
use crate::menu::*;

const DEFAULT_STROKE_WIDTH: f64 = 2.0;
const DEFAULT_SIZE: f64 = 24.0;

#[component]
pub fn IconsView() -> impl IntoView {
    let (icon_filter, set_icon_filter) = signal("".to_string());
    let (selected_icon, set_selected_icon) = signal(None::<LucideGlyph>);

    // Customizer state (None = use currentColor, adapts to theme)
    let (icon_color, set_icon_color) = signal(None::<String>);
    let (icon_stroke_width, set_icon_stroke_width) = signal(DEFAULT_STROKE_WIDTH);
    let (icon_size, set_icon_size) = signal(DEFAULT_SIZE);
    let (absolute_stroke, set_absolute_stroke) = signal(false);

    let clear_filter = move |_| set_icon_filter.set("".to_string());

    // close detail panel when filter changes
    Effect::new(move |_| {
        icon_filter.get();
        set_selected_icon.set(None);
    });

    // close detail panel on Escape key
    window_event_listener(keydown, move |ev: KeyboardEvent| {
        if ev.key() == "Escape" {
            set_selected_icon.set(None);
        }
    });

    // Debounced filter: updates 150ms after last keystroke
    let (debounced_filter, set_debounced_filter) = signal("".to_string());
    let on_input = move |ev: Event| {
        let value = event_target_value(&ev);
        set_icon_filter.set(value.clone());
        set_timeout(
            move || {
                if icon_filter.get_untracked() == value {
                    set_debounced_filter.set(icon_filter.get_untracked());
                }
            },
            std::time::Duration::from_millis(150),
        );
    };

    // Sync debounced filter on programmatic changes (category click, clear)
    Effect::new(move |prev: Option<String>| {
        let current = icon_filter.get();
        if prev.as_ref() != Some(&current) {
            set_debounced_filter.set(current.clone());
        }
        current
    });

    let icon_count = LucideGlyph::count();

    view! {
        <div class="flex flex-row">
            // icon group list
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

            // searchable icon table
            <div class="px-10 mt-5 flex flex-col flex-auto h-screen overflow-y-auto overflow-x-hidden scroll-pb-72">
                <StickyTop class="bg-gradient-to-b from-85% from-background to-100% to-transparent">
                    <MainMenu class="justify-end text-primary"/>
                    <div class = "flex flex-row overflow-y-auto items-center w-full p-2 px-4 my-6 bg-secondary rounded-lg border border-transparent focus-within:border-highlight/80 transition-colors">
                        <Icon glyph=LucideGlyph::Search />
                        <input type="text"
                               class="flex-auto p-2 bg-transparent focus:outline-none  focus:border-1"
                               prop:placeholder="Search icons..."
                               prop:value={move || icon_filter.get()}
                               on:input=on_input
                        />
                        <Icon glyph=LucideGlyph::X class="cursor-pointer" on:click=clear_filter />
                    </div>
                </StickyTop>
                <IconGrid
                    filter=debounced_filter
                    selected=selected_icon
                    on_select=Callback::new(move |g| set_selected_icon.set(Some(g)))
                    class="flex flex-row flex-wrap gap-2"
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
        </div>
        <AnimationStyles />
        <IconDetail selected_icon=selected_icon set_selected_icon=set_selected_icon />
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

#[component]
fn IconDetail(
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

    // Reset menus and animation when selected icon changes
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
                <div class="fixed bottom-0 left-64 right-0 bg-secondary border-t border-primary/20 p-6 flex flex-row gap-8 items-start z-50"
                     on:click=move |_| { set_svg_menu_open.set(false); set_jsx_menu_open.set(false); }>
                    // large icon preview
                    <div class="flex-none w-56 h-56 flex items-center justify-center rounded-xl"
                         style="background-image: linear-gradient(to right, rgba(128,128,128,0.15) 1px, transparent 1px), linear-gradient(to bottom, rgba(128,128,128,0.15) 1px, transparent 1px); background-size: calc(200px / 24) calc(200px / 24); background-position: 12px 12px;">
                        <div class=move || {
                            let idx = anim_type.get();
                            if idx >= 2 { format!("text-primary {}", ANIM_TYPES[idx].1) } else { "text-primary".to_string() }
                        }>
                            {move || {
                                replay_key.get();
                                if anim_type.get() == 1 {
                                    {
                                        let d = draw_duration.get();
                                        let dl = draw_delay.get();
                                        view! { <DrawIcon glyph=icon size="200" stroke_width="2" duration_ms=d delay_ms=dl /> }.into_any()
                                    }
                                } else {
                                    view! { <Icon glyph=icon size="200" stroke_width="2" /> }.into_any()
                                }
                            }}
                        </div>
                    </div>

                    // info
                    <div class="flex-auto flex flex-col gap-3 min-w-0">
                        <div class="flex flex-row items-center gap-4">
                            <span class="text-xl font-medium text-primary">{name.clone()}</span>
                        </div>

                        {if !tags.is_empty() {
                            let tag_str = tags.join(" \u{2022} ");
                            Some(view! { <p class="text-sm text-primary/60">{tag_str}</p> })
                        } else {
                            None
                        }}

                        {if !categories.is_empty() {
                            Some(view! {
                                <div class="flex flex-row gap-2 flex-wrap">
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

                        // actions
                        <div class="flex flex-row gap-3 pt-1">
                            // Copy SVG dropdown
                            <div class="relative">
                                <button
                                    class="px-4 py-1.5 text-sm rounded-lg border border-primary/20 text-primary/70 hover:bg-primary/10 flex items-center gap-2"
                                    aria-haspopup="menu"
                                    aria-expanded=move || svg_menu_open.get().to_string()
                                    on:click=move |ev: web_sys::MouseEvent| { ev.stop_propagation(); set_svg_menu_open.set(!svg_menu_open.get()); set_jsx_menu_open.set(false); }
                                >
                                    <Icon glyph=Signal::derive(move || {
                                        if svg_copied.get() { LucideGlyph::Check } else { LucideGlyph::Copy }
                                    }) size="16" />
                                    {move || if svg_copied.get() { "Copied!" } else { "Copy SVG" }}
                                    <Icon glyph=LucideGlyph::ChevronUp size="14" />
                                </button>
                                {move || svg_menu_open.get().then(|| {
                                    let svg_for_copy = full_svg.clone();
                                    let svg_for_data_url = full_svg.clone();
                                    let svg_for_download = full_svg.clone();
                                    let svg_for_png = full_svg.clone();
                                    let name_for_svg = icon_name.clone();
                                    let name_for_png = icon_name.clone();
                                    view! {
                                        <div class="absolute bottom-full left-0 mb-1 bg-background border border-primary/20 rounded-lg shadow-lg py-1 min-w-[160px] z-50" role="menu">
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_and_flash(&svg_for_copy, set_svg_copied, set_svg_menu_open);
                                                    }>
                                                "Copy SVG"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        let data_url = format!("data:image/svg+xml,{}", js_sys::encode_uri_component(&svg_for_data_url));
                                                        copy_and_flash(&data_url, set_svg_copied, set_svg_menu_open);
                                                    }>
                                                "Copy Data URL"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        download_blob(&svg_for_download, &format!("{}.svg", name_for_svg), "image/svg+xml");
                                                        set_svg_menu_open.set(false);
                                                    }>
                                                "Download SVG"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
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
                            // Copy JSX dropdown
                            <div class="relative">
                                <button
                                    class="px-4 py-1.5 text-sm rounded-lg border border-primary/20 text-primary/70 hover:bg-primary/10 flex items-center gap-2"
                                    aria-haspopup="menu"
                                    aria-expanded=move || jsx_menu_open.get().to_string()
                                    on:click=move |ev: web_sys::MouseEvent| { ev.stop_propagation(); set_jsx_menu_open.set(!jsx_menu_open.get()); set_svg_menu_open.set(false); }
                                >
                                    {move || if jsx_copied.get() { "Copied!" } else { "Copy JSX" }}
                                    <Icon glyph=LucideGlyph::ChevronUp size="14" />
                                </button>
                                {move || jsx_menu_open.get().then(|| {
                                    let comp = component_name;
                                    let kebab = kebab_name.clone();
                                    let kebab2 = kebab_name.clone();
                                    view! {
                                        <div class="absolute bottom-full left-0 mb-1 bg-background border border-primary/20 rounded-lg shadow-lg py-1 min-w-[180px] z-50" role="menu">
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_and_flash(&format!("<{} />", comp), set_jsx_copied, set_jsx_menu_open);
                                                    }>
                                                "Copy JSX"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_and_flash(comp, set_jsx_copied, set_jsx_menu_open);
                                                    }>
                                                "Copy Component Name"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_and_flash(&format!("<{} />", kebab), set_jsx_copied, set_jsx_menu_open);
                                                    }>
                                                "Copy Vue"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_and_flash(&format!("<{} />", comp), set_jsx_copied, set_jsx_menu_open);
                                                    }>
                                                "Copy Svelte"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_and_flash(&format!("<lucide-angular name=\"{}\" />", kebab2), set_jsx_copied, set_jsx_menu_open);
                                                    }>
                                                "Copy Angular"
                                            </button>
                                            <button role="menuitem" class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
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
                        // animation type pills
                        <hr class="border-primary/10"/>
                        <div class="flex flex-row flex-wrap gap-1 pt-1 items-center">
                            <span class="text-xs text-primary/40 mr-1">"Animate:"</span>
                            {ANIM_TYPES.iter().enumerate().map(|(i, (label, _))| {
                                let is_sel = move || anim_type.get() == i;
                                view! {
                                    <button
                                        class=move || if is_sel() {
                                            "px-2 py-0.5 text-xs rounded-full border border-highlight/80 bg-highlight/10 text-highlight font-medium cursor-pointer"
                                        } else {
                                            "px-2 py-0.5 text-xs rounded-full border border-primary/20 text-primary/50 hover:bg-primary/10 cursor-pointer"
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
                                    class="px-2 py-0.5 text-xs rounded-full border border-primary/20 text-primary/50 hover:bg-primary/10 flex items-center gap-1 cursor-pointer"
                                    on:click=move |ev: web_sys::MouseEvent| {
                                        ev.stop_propagation();
                                        set_replay_key.update(|k| *k += 1);
                                    }
                                >
                                    <Icon glyph=LucideGlyph::RotateCcw size="10" />
                                    "Replay"
                                </button>
                            })}
                        </div>
                        // Draw-In controls
                        {move || (anim_type.get() == 1).then(|| view! {
                            <div class="flex flex-row gap-4 items-center">
                                <div class="flex flex-row items-center gap-2 flex-1">
                                    <label class="text-xs text-primary/40 whitespace-nowrap">"Duration"</label>
                                    <input type="range" class="w-full" min="100" max="2000" step="100"
                                        prop:value=move || draw_duration.get().to_string()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                                set_draw_duration.set(v);
                                                set_replay_key.update(|k| *k += 1);
                                            }
                                        }
                                    />
                                    <span class="text-xs text-primary/50 w-12 text-right">{move || format!("{}ms", draw_duration.get())}</span>
                                </div>
                                <div class="flex flex-row items-center gap-2 flex-1">
                                    <label class="text-xs text-primary/40 whitespace-nowrap">"Delay"</label>
                                    <input type="range" class="w-full" min="0" max="1000" step="100"
                                        prop:value=move || draw_delay.get().to_string()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                                set_draw_delay.set(v);
                                                set_replay_key.update(|k| *k += 1);
                                            }
                                        }
                                    />
                                    <span class="text-xs text-primary/50 w-12 text-right">{move || format!("{}ms", draw_delay.get())}</span>
                                </div>
                            </div>
                        })}
                    </div>

                    // close button
                    <button class="flex-none p-1 text-primary/50 hover:text-primary" on:click=dismiss>
                        <Icon glyph=LucideGlyph::X />
                    </button>
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
