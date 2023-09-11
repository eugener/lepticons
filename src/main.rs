use leptos::*;
use leptos_meta::*;
use lucide_icons::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}


#[component]
fn App(cx: Scope) -> impl IntoView {

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <div class="m-5 flex flex-row  flex-wrap gap-2">

            {
                ALL_ICONS.iter().map(|it|
                    view! { cx,
                        <div class="p-3.5 bg-gray-100 rounded-lg hover:bg-gray-200 border border-gray-100 hover:border-gray-400/50 hover:border-1 group">
                            <Icon kind={*it}/>
                            <div class="absolute group:top-0.95 group:left-1/2 -translate-x-1/2 -translate-y-5/6 z-1 opacity-0 transition-opacity group-hover:opacity-100 p-1 px-2 text-xs text-white bg-orange-700 rounded" >
                               {it.name}
                            </div>
                        </div>

                    }
                ).collect::<Vec<_>>()

            }
        </div>
    }
}

// fn Tooltip(cs:Scope) -> impl IntoView {
//    view! { cx,
//       <div class="tooltip">Hover over me
//         <span class="tooltiptext">Tooltip text</span>
//       </div>
//    }
// }

