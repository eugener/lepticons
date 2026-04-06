use convert_case::{Case, Casing};
use leptos::ev::*;
use leptos::logging::*;
use leptos::prelude::*;

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

    let on_input = move |ev: Event| {
        set_icon_filter.set(event_target_value(&ev));
        log!("Filter: {}", icon_filter.get_untracked());
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
            <div class="px-10 mt-5 flex flex-col flex-auto h-screen overflow-y-auto">
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
                <IconTable icon_filter=icon_filter set_selected_icon=set_selected_icon />
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
    set_selected_icon: WriteSignal<Option<LucideGlyph>>,
) -> impl IntoView {
    let filtered_icons = move || LucideGlyph::find(icon_filter.get().to_lowercase().as_str());

    view! {
        <div class="flex flex-row flex-wrap gap-2">
        {
            move || filtered_icons().iter().map( |icon| {
                let icon_for_click = icon.clone();
                view!{ <IconCell icon=icon.clone() on:click=move |_| set_selected_icon.set(Some(icon_for_click.clone())) /> }
            }).collect::<Vec<_>>()
        }
        </div>
    }
}

const ICON_CONTAINER_STYLE: &str = "relative group p-2 bg-secondary rounded-lg hover:bg-primary/20 border border-primary/0 hover:border-primary/100 cursor-pointer";
const TOOLTIP_STYLE: &str = "absolute left-1/2 -translate-x-1/2 -bottom-4 z-10 opacity-0 transition-opacity group-hover:opacity-100 py-0.5 px-1 text-[0.5rem] font-light text-white bg-orange-700/90 border border-orange-750/90 rounded whitespace-nowrap";

#[component]
fn IconCell(icon: LucideGlyph) -> impl IntoView {
    let glyph = icon.clone();
    view! {
        <div class=ICON_CONTAINER_STYLE>
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

    view! {
        {move || selected_icon.get().map(|icon| {
            let name = display_name(&icon);
            let tags = icon.tags().into_iter().filter(|t| !t.is_empty()).collect::<Vec<_>>();
            let categories = icon.categories().into_iter().filter(|c| !c.is_empty()).collect::<Vec<_>>();
            let glyph = icon.clone();

            view! {
                <div class="fixed bottom-0 left-64 right-0 bg-secondary border-t border-primary/20 p-6 flex flex-row gap-8 items-start z-50">
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
                            <span class="text-xl font-medium text-primary">{name}</span>
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
