use leptos::ev::*;
use leptos::logging::*;
use leptos::*;
use leptos::{
    component, create_signal, event_target_value, IntoView, ReadSignal, SignalGet, SignalSet,
};

use lepticons::LucideGlyph;
use lepticons::*;
use strum::IntoEnumIterator;

use crate::components::*;
use crate::menu::*;
use std::sync::OnceLock;

static ICON_COUNT: OnceLock<usize> = OnceLock::new();

#[component]
pub fn IconsView() -> impl IntoView {
    let (icon_filter, set_icon_filter) = create_signal("".to_string());

    let clear_filter = move |_| set_icon_filter.set("".to_string());

    let on_input = move |ev: Event| {
        set_icon_filter.set(event_target_value(&ev));
        log!("Filter: {}", icon_filter.get_untracked());
    };

    let icon_count = ICON_COUNT.get_or_init(|| LucideGlyph::iter().count());

    view! {
        // <Meta name="color-scheme" content="light"/>
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
            <div class="px-10 mt-5 flex flex-col gap-4 flex-auto h-screen overflow-y-auto">
                <StickyTop class="bg-gradient-to-b from-85% from-background to-100% to-transparent">
                    <MainMenu class="justify-end text-primary"/>
                    <div class = "flex flex-row overflow-y-auto items-center w-full focus:border-orange-700/50 p-2 px-4 my-6 bg-secondary rounded-lg">
                        <Icon glyph= move || LucideGlyph::Search/>
                        <input type="text"
                               class="flex-auto p-2 bg-transparent focus:outline-none  focus:border-1"
                               prop:placeholder="Search icons..."
                               prop:value={move || icon_filter.get()}
                               on:input=on_input
                        />
                        <Icon glyph=move || LucideGlyph::X class="cursor-pointer" on:click=clear_filter />
                    </div>
                </StickyTop>
                <IconTable icon_filter=icon_filter />
            </div>
        </div>
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
fn IconTable(icon_filter: ReadSignal<String>) -> impl IntoView {
    let filtered_icons = move || LucideGlyph::find(icon_filter.get().to_lowercase().as_str());

    view! {
        <div class="flex flex-row flex-wrap gap-2 justify-between">
        {
            move || filtered_icons().iter().map( |icon|
                view!{ <IconCell icon=icon.clone()/> }
            ).collect::<Vec<_>>()
        }
        </div>
    }
}

const ICON_CONTAINER_STYLE: &'static str = "relative group p-3.5 bg-secondary rounded-lg hover:bg-primary/20 border-1 border-primary/0 hover:border-primary/100 hover:border-1";
const TOOLTIP_STYLE: &'static str = "absolute left-1/2 -translate-x-1/2 translate-y-5 z-10 opacity-0 transition-opacity group-hover:opacity-100 p-1 px-2 text-xs font-light text-white bg-orange-700/90 border border-1 border-orange-750/90 rounded";

#[component]
fn IconCell(icon: LucideGlyph) -> impl IntoView {
    let glyph = icon.clone();
    view! {
        <div class=ICON_CONTAINER_STYLE>
            <Icon<LucideGlyph> glyph= move || glyph.clone() /> // stroke_width={1.7} stroke="#645e5f"/>
            <div class=TOOLTIP_STYLE >
               {icon.name()}
            </div>
        </div>
    }
}
