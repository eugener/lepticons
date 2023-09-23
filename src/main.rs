use std::iter::Iterator;

use leptos::*;
use leptos::ev::*;
use leptos::logging::log;
use leptos_meta::*;
use lucide_icons::*;
use strum::IntoEnumIterator;

//TODO show and select icon attributes
//     show and select categories
//     main menu

fn main() {
    provide_meta_context();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {

    let (icon_filter, set_icon_filter) = create_signal( "".to_string());

    let clear_filter = move |_| { set_icon_filter.set("".to_string())};

    let on_input = move |ev: Event| {
        set_icon_filter.set( event_target_value(&ev));
        log!("Filter: {}",icon_filter.get_untracked());
    };

    view! {
        <div class="m-5 flex flex-col gap-4">
        <div class = "flex flex-row overflow-y-auto items-center w-full sticky top-0 z-50 focus:border-orange-700/50 p-2 px-4 my-6 bg-gray-100 rounded-lg">
            <Icon icon={LucideIcon::Search}/>
            <input type="text"
                   class="flex-auto p-2 bg-transparent focus:outline-none  focus:border-1"
                   // _ref=input_ref
                   prop:placeholder="Search icons..."
                   prop:value={move || icon_filter.get()}
                   on:input=on_input
            />
            <Icon icon={LucideIcon::X} class="cursor-pointer" on:click=clear_filter />
        </div>
            <IconTable icon_filter=icon_filter />
        </div>
    }
}

#[component]
fn IconTable(
    icon_filter: ReadSignal<String>,
) -> impl IntoView {

    let filter = move || icon_filter.get().to_lowercase();

    let filtered_icons = move || {

        if filter().is_empty() {
            return LucideIcon::iter().collect::<Vec<_>>();
        }

        let f = filter().to_lowercase();
        LucideIcon::iter()
            .filter(|icon| icon.name().to_lowercase().contains(&f))
            .collect::<Vec<_>>()

    };

    const ICON_CONTAINER:  &'static str = "relative p-3.5 bg-gray-100 rounded-lg hover:bg-gray-200 border border-gray-100 hover:border-gray-400/50 hover:border-1 group";
    const TOOLTIP:  &'static str = "absolute left-1/2 -translate-x-1/2 translate-y-5 z-10 opacity-0 transition-opacity group-hover:opacity-100 p-1 px-2 text-xs font-light text-white bg-orange-700/90 border border-1 border-orange-750/90 rounded";

    view! {

        <div class="flex flex-row flex-wrap gap-2 justify-between">
        {
            move || filtered_icons().iter().map( |icon|
                view! {
                    <div class=ICON_CONTAINER>
                        <Icon icon={icon.clone()} /> // stroke_width={1.7} stroke="#645e5f"/>
                        <div class=TOOLTIP >
                           {icon.name()}
                        </div>
                    </div>
                }
            ).collect::<Vec<_>>()
        }
        </div>

    }
}
