use leptos::logging::log;
use leptos::*;
use leptos_meta::*;
use lucide_icons::*;
use strum::IntoEnumIterator;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (icon_filter, set_icon_filter) = create_signal( "".to_string());

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>

        <div class="m-5 flex flex-col gap-4">
        <div class = "flex flex-row items-center w-full focus:border-orange-700/50 p-2 px-4 bg-gray-100 rounded-lg">
            <Icon icon={LucideIcon::Search}/>
            <input type="text"
                   class="flex-auto p-2 bg-transparent focus:outline-none  focus:border-1"
                   prop:placeholder="Search icons..."
                   prop:value={move || icon_filter.get()}
                   on:input={move |ev| {
                      set_icon_filter.set( event_target_value(&ev));
                      log!("Value: {}", event_target_value(&ev));
                   }}
            />
            <Icon icon={LucideIcon::X} on:click={move |ev| {
                      set_icon_filter.set( "".to_string());
                      // log!("Value: {}", event_target_value(&ev));
                   }}/>
        </div>
            {move || icon_filter.get()}
            // <IconTable icons={filtered_icons} />
            <IconTable icon_filter={icon_filter.get()} />
        // <IconTable icon_filter={(move || icon_filter.get())()} />
        </div>
    }
}

#[component]
fn IconTable(
    //icons: Vec<&'static IconType<'static>>
    icon_filter: String,
    //&'static [IconType<'static>],
) -> impl IntoView {
    log!("Value: {}", icon_filter);

    let filter = icon_filter.to_lowercase();
    let filtered_icons: Vec<LucideIcon> = match filter.as_str() {
        "" => LucideIcon::iter().collect(),
        _ => LucideIcon::iter()
            .filter(|icon| icon.to_string().to_lowercase().contains(&filter))
            .collect(),
    };

    view! {

        <div class="flex flex-row flex-wrap gap-2">
        {
            filtered_icons.iter().cloned().map( |icon|
                view! {
                    <div class="relative p-3.5 bg-gray-100 rounded-lg hover:bg-gray-200 border border-gray-100 hover:border-gray-400/50 hover:border-1 group">
                        <Icon icon={icon.clone()}/>
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
