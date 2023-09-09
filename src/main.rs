mod tools;

use leptos::*;
use leptos_meta::*;
use lucid_icons::*;
use tools::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    // list_icons();
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {

    // let (count, _set_count) = create_signal(cx, 5);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <div class="m-5 flex flex-row  flex-wrap gap-2">

            {
                LucidIconType::iter().map(|it| {
                    view! { cx,
                        <div class="p-4 bg-gray-100 rounded-lg hover:bg-gray-200">
                            <LucidIcon icon_type={it} />
                        </div>
                    }
                }).collect::<Vec<_>>()

            }
        </div>
    }
}

 #[component]
pub fn Counter( cx: Scope ) -> impl IntoView {

    let (count, set_count) = create_signal(cx, 0 );

    view! { cx,
        <button
            class="p-6 m-4 text-sm bg-gray-200 rounded-lg shadow-md hover:bg-gray-300 gap-2"
            on:click=move |_| {
                set_count.set( count.get() + 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}


