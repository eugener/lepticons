use convert_case::{Case, Casing};
use leptos::ev::*;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::js_sys;
use web_sys::wasm_bindgen;

use lepticons::LucideGlyph;
use lepticons::*;
use strum::IntoEnumIterator;

use crate::components::*;
use crate::menu::*;
use std::sync::OnceLock;

static ICON_COUNT: OnceLock<usize> = OnceLock::new();

#[component]
pub fn IconsView() -> impl IntoView {
    let (icon_filter, set_icon_filter) = signal("".to_string());
    let (selected_icon, set_selected_icon) = signal(None::<LucideGlyph>);

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

    let on_input = move |ev: Event| {
        set_icon_filter.set(event_target_value(&ev));
    };

    let icon_count = ICON_COUNT.get_or_init(|| LucideGlyph::iter().count());

    view! {
        <div class="flex flex-row">
            // icon group list
            <div class="w-64 flex-none bg-secondary h-screen overflow-y-auto">

                <StickyTop class="px-10 bg-gradient-to-b from-95% from-secondary to-100% to-transparent">
                    <div class="flex flex-col items-center gap-0 cursor-pointer">
                        <img src="lepticons.png" class="pt-5 w-48"/>
                        <p class="text-primary text-[0.7rem] pb-2 self-end" on:click=clear_filter >
                           {format!("{} icons", icon_count)}
                        </p>
                   </div>
                   <hr/>
                </StickyTop>

                <div class="px-10 pt-5 flex flex-col gap-2">
                   {
                       move || LucideGlyph::all_categories().iter()
                             .filter(|(k, _)| !k.is_empty())
                             .map(|(title, count)| {
                                  let title_cloned = title.clone();
                                  view! {
                                      <IconGroupLabel title=title.to_string() count=*count
                                      on:click=move |_| set_icon_filter.set(title_cloned.to_string()) />
                                  }
                              }).collect::<Vec<_>>()
                   }
                </div>

            </div>

            // searchable icon table
            <div class="px-10 mt-5 flex flex-col flex-auto h-screen overflow-y-auto overflow-x-hidden">
                <StickyTop class="bg-gradient-to-b from-85% from-background to-100% to-transparent">
                    <MainMenu class="justify-end text-primary"/>
                    <div class = "flex flex-row overflow-y-auto items-center w-full focus:border-orange-700/50 p-2 px-4 my-6 bg-secondary rounded-lg">
                        <Icon glyph=Signal::derive(move || LucideGlyph::Search)/>
                        <input type="text"
                               class="flex-auto p-2 bg-transparent focus:outline-none  focus:border-1"
                               prop:placeholder="Search icons..."
                               prop:value={move || icon_filter.get()}
                               on:input=on_input
                        />
                        <Icon glyph=Signal::derive(move || LucideGlyph::X) class="cursor-pointer" on:click=clear_filter />
                    </div>
                </StickyTop>
                <IconTable icon_filter=icon_filter selected_icon=selected_icon set_selected_icon=set_selected_icon />
            </div>
        </div>
        <IconDetail selected_icon=selected_icon set_selected_icon=set_selected_icon />
    }
}

#[component]
fn IconGroupLabel(title: String, count: u16) -> impl IntoView {
    view! {
      <div class="flex flex-row gap-4 text-sm text-primary/70 cursor-pointer">
         <div class="flex-auto">
             {title}
         </div>
        <div class="flex-none text-primary/50 text-xs">{format!("{}", count)}</div>
      </div>
    }
}

#[component]
fn IconTable(
    icon_filter: ReadSignal<String>,
    selected_icon: ReadSignal<Option<LucideGlyph>>,
    set_selected_icon: WriteSignal<Option<LucideGlyph>>,
) -> impl IntoView {
    let filtered_icons = move || LucideGlyph::find(icon_filter.get().to_lowercase().as_str());

    view! {
        <div class="flex flex-row flex-wrap gap-2">
        {
            // Only re-run when filter changes, not on selection change
            move || filtered_icons().iter().map( |icon| {
                let icon_for_click = icon.clone();
                let icon_for_sel = icon.clone();
                // Per-cell derived signal: only the affected cells re-render on selection change
                let is_selected = Signal::derive(move || selected_icon.get().as_ref() == Some(&icon_for_sel));
                view!{ <IconCell icon=icon.clone() selected=is_selected on:click=move |_| set_selected_icon.set(Some(icon_for_click.clone())) /> }
            }).collect::<Vec<_>>()
        }
        </div>
    }
}

const ICON_STYLE: &str = "relative group p-2 bg-secondary rounded-lg hover:bg-primary/20 border border-transparent cursor-pointer";
const ICON_STYLE_SELECTED: &str = "relative group p-2 bg-primary/10 rounded-lg border border-orange-700/80 cursor-pointer";
const TOOLTIP_STYLE: &str = "absolute left-1/2 -translate-x-1/2 -bottom-4 z-10 opacity-0 transition-opacity group-hover:opacity-100 py-0.5 px-1 text-[0.5rem] font-light text-white bg-orange-700/90 border border-orange-750/90 rounded whitespace-nowrap";

#[component]
fn IconCell(icon: LucideGlyph, selected: Signal<bool>) -> impl IntoView {
    let glyph = icon.clone();
    let style = move || if selected.get() { ICON_STYLE_SELECTED } else { ICON_STYLE };
    view! {
        <div class=style>
            <Icon<LucideGlyph> glyph=Signal::derive(move || glyph.clone()) />
            <div class=TOOLTIP_STYLE >
               {icon.name()}
            </div>
        </div>
    }
}

/// Formats an enum name like "BatteryCharging" to kebab-case "battery-charging".
fn display_name(icon: &LucideGlyph) -> String {
    icon.name().to_case(Case::Kebab)
}

#[component]
fn IconDetail(
    selected_icon: ReadSignal<Option<LucideGlyph>>,
    set_selected_icon: WriteSignal<Option<LucideGlyph>>,
) -> impl IntoView {
    let dismiss = move |_| set_selected_icon.set(None);

    // Signals live at component scope, not inside the reactive closure
    let (svg_menu_open, set_svg_menu_open) = signal(false);
    let (jsx_menu_open, set_jsx_menu_open) = signal(false);
    let (copied, set_copied) = signal(false);

    // Reset menus when selected icon changes
    Effect::new(move |_| {
        selected_icon.get();
        set_svg_menu_open.set(false);
        set_jsx_menu_open.set(false);
        set_copied.set(false);
    });

    view! {
        {move || selected_icon.get().map(|icon| {
            let name = display_name(&icon);
            let tags = icon.tags().into_iter().filter(|t| !t.is_empty()).collect::<Vec<_>>();
            let categories = icon.categories().into_iter().filter(|c| !c.is_empty()).collect::<Vec<_>>();
            let glyph = icon.clone();
            let svg_content = icon.svg();
            let full_svg = format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\">{}</svg>",
                svg_content
            );

            let icon_name = name.clone();
            let component_name = icon.name(); // PascalCase e.g. "ShieldBan"
            let kebab_name = name.clone();

            view! {
                <div class="fixed bottom-0 left-64 right-0 bg-secondary border-t border-primary/20 p-6 flex flex-row gap-8 items-start z-50"
                     on:click=move |_| { set_svg_menu_open.set(false); set_jsx_menu_open.set(false); }>
                    // large icon preview with grid background
                    <div class="flex-none w-56 h-56 flex items-center justify-center rounded-xl"
                         style="background-image: linear-gradient(to right, rgba(128,128,128,0.15) 1px, transparent 1px), linear-gradient(to bottom, rgba(128,128,128,0.15) 1px, transparent 1px); background-size: calc(200px / 24) calc(200px / 24); background-position: 12px 12px;">
                        <div class="text-primary">
                            <Icon<LucideGlyph> glyph=Signal::derive(move || glyph.clone()) size="200" stroke_width="2" />
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
                            <div class="relative">
                                <button class="px-4 py-1.5 text-sm rounded-lg border border-primary/20 text-primary/70 hover:bg-primary/10 flex items-center gap-2"
                                        on:click=move |ev: web_sys::MouseEvent| { ev.stop_propagation(); set_svg_menu_open.set(!svg_menu_open.get()); set_jsx_menu_open.set(false); }>
                                    <Icon glyph=Signal::derive(move || {
                                        if copied.get() { LucideGlyph::Check } else { LucideGlyph::Copy }
                                    }) size="16" />
                                    {move || if copied.get() { "Copied!" } else { "Copy SVG" }}
                                    <Icon glyph=Signal::derive(move || LucideGlyph::ChevronUp) size="14" />
                                </button>
                                {move || svg_menu_open.get().then(|| {
                                    let svg_for_copy = full_svg.clone();
                                    let svg_for_data_url = full_svg.clone();
                                    let svg_for_download = full_svg.clone();
                                    let svg_for_png = full_svg.clone();
                                    let name_for_svg = icon_name.clone();
                                    let name_for_png = icon_name.clone();
                                    view! {
                                        <div class="absolute bottom-full left-0 mb-1 bg-background border border-primary/20 rounded-lg shadow-lg py-1 min-w-[160px] z-50">
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_to_clipboard(&svg_for_copy);
                                                        set_copied.set(true);
                                                        set_svg_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy SVG"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        let data_url = format!("data:image/svg+xml,{}", js_sys::encode_uri_component(&svg_for_data_url));
                                                        copy_to_clipboard(&data_url);
                                                        set_copied.set(true);
                                                        set_svg_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy Data URL"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        download_blob(&svg_for_download, &format!("{}.svg", name_for_svg), "image/svg+xml");
                                                        set_svg_menu_open.set(false);
                                                    }>
                                                "Download SVG"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
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
                                <button class="px-4 py-1.5 text-sm rounded-lg border border-primary/20 text-primary/70 hover:bg-primary/10 flex items-center gap-2"
                                        on:click=move |ev: web_sys::MouseEvent| { ev.stop_propagation(); set_jsx_menu_open.set(!jsx_menu_open.get()); set_svg_menu_open.set(false); }>
                                    {move || if copied.get() { "Copied!" } else { "Copy JSX" }}
                                    <Icon glyph=Signal::derive(move || LucideGlyph::ChevronUp) size="14" />
                                </button>
                                {move || jsx_menu_open.get().then(|| {
                                    let comp = component_name.clone();
                                    let comp2 = component_name.clone();
                                    let comp3 = component_name.clone();
                                    let comp5 = component_name.clone();
                                    let kebab = kebab_name.clone();
                                    let kebab2 = kebab_name.clone();
                                    view! {
                                        <div class="absolute bottom-full left-0 mb-1 bg-background border border-primary/20 rounded-lg shadow-lg py-1 min-w-[180px] z-50">
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_to_clipboard(&format!("<{} />", comp));
                                                        set_copied.set(true);
                                                        set_jsx_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy JSX"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_to_clipboard(&comp2);
                                                        set_copied.set(true);
                                                        set_jsx_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy Component Name"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_to_clipboard(&format!("<{} />", kebab));
                                                        set_copied.set(true);
                                                        set_jsx_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy Vue"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_to_clipboard(&format!("<{} />", comp3));
                                                        set_copied.set(true);
                                                        set_jsx_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy Svelte"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_to_clipboard(&format!("<lucide-angular name=\"{}\" />", kebab2));
                                                        set_copied.set(true);
                                                        set_jsx_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy Angular"
                                            </button>
                                            <button class="w-full text-left px-4 py-2 text-sm text-primary hover:bg-primary/10"
                                                    on:click=move |_| {
                                                        copy_to_clipboard(&format!("<Icon<LucideGlyph> glyph=Signal::derive(move || LucideGlyph::{}) />", comp5));
                                                        set_copied.set(true);
                                                        set_jsx_menu_open.set(false);
                                                        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                    }>
                                                "Copy Leptos"
                                            </button>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    </div>

                    // close button
                    <button class="flex-none p-1 text-primary/50 hover:text-primary" on:click=dismiss>
                        <Icon glyph=Signal::derive(move || LucideGlyph::X) />
                    </button>
                </div>
            }
        })}
    }
}

fn copy_to_clipboard(text: &str) {
    if let Some(w) = web_sys::window() {
        let clipboard = w.navigator().clipboard();
        let _ = clipboard.write_text(text);
    }
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

    let img = web_sys::HtmlImageElement::new().unwrap();
    let data_url = format!("data:image/svg+xml,{}", js_sys::encode_uri_component(svg_str));

    let filename = format!("{}.png", name);
    let canvas_clone = canvas.clone();
    let img_clone = img.clone();
    let body_clone = body;

    // Use Closure::once_into_js to avoid memory leak from closure.forget()
    let closure = wasm_bindgen::closure::Closure::once(move || {
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &img_clone, 0.0, 0.0, png_size as f64, png_size as f64,
        ).unwrap();
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

    let cb: js_sys::Function = closure.into_js_value().unchecked_into();
    img.set_onload(Some(&cb));
    img.set_src(&data_url);
}
