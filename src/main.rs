use std::iter::Iterator;
use leptos::logging::log;
use leptos::*;
use leptos::ev::{Event};
use leptos::html::Input;
use leptos_meta::*;
use lucide_icons::*;
use strum::IntoEnumIterator;

fn main() {
    provide_meta_context();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {

    let (icon_filter, set_icon_filter) = create_signal( "".to_string());

    let input_ref = create_node_ref::<Input>();

    let clear_filter = move |_| { set_icon_filter.set("".to_string())};

    let on_input = move |ev: Event| {
        set_icon_filter.set( event_target_value(&ev));
        log!("Filter: {}",icon_filter.get_untracked());
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>

        <div class="m-5 flex flex-col gap-4">
        <div class = "flex flex-row items-center w-full focus:border-orange-700/50 p-2 px-4 bg-gray-100 rounded-lg">
            <Icon icon={LucideIcon::Search}/>
            <input type="text"
                   class="flex-auto p-2 bg-transparent focus:outline-none  focus:border-1"
                   _ref=input_ref
                   prop:placeholder="Search icons..."
                   prop:value={move || icon_filter.get()}
                   on:input=on_input
            />
            <Icon icon={LucideIcon::X} class="cursor-pointer" on:click=clear_filter />
        </div>
            // {move || icon_filter.get()}
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

        let f = filter().to_lowercase();
        match f.as_str() {
            "" => return LucideIcon::iter().collect(),
            _ =>  LucideIcon::iter()
                .filter(|icon| icon.to_string().to_lowercase().contains(&f))
                .collect::<Vec<_>>()
        }

    };


    view! {

        <div class="flex flex-row flex-wrap gap-2 justify-between">
        {
            move || filtered_icons().iter().cloned().map( |icon|
                view! {
                    <div class="relative p-3.5 bg-gray-100 rounded-lg hover:bg-gray-200 border border-gray-100 hover:border-gray-400/50 hover:border-1 group">
                        <Icon icon={icon.clone()} stroke_width={1.7} stroke="#645e5f"/>
                        <div class="absolute left-1/2 -translate-x-1/2 translate-y-5 z-50 opacity-0 transition-opacity group-hover:opacity-100 p-1 px-2 text-xs font-light text-white bg-orange-700/90 rounded" >
                           {icon.to_string()}
                        </div>
                    </div>
                }
            ).collect::<Vec<_>>()
        }
        </div>

    }
}
