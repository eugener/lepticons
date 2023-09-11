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
                ALL_ICONS.iter().map(|it| {

                    view! { cx,
                        <div class="p-4 bg-gray-100 rounded-lg hover:bg-gray-200">
                            <Icon kind={*it}/>
                        </div>

                    }
                }).collect::<Vec<_>>()

            }
        </div>
    }
}

