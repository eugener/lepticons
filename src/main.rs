use leptos::*;
use leptos_meta::*;
use lucide_icons::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}




#[component]
fn App(cx: Scope) -> impl IntoView {


    // let (count, _set_count) = create_signal(cx, 5);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <div class="m-5 flex flex-row  flex-wrap gap-2">

            {
                // LucideIconType::iter().map(|it| {
                //   icons.iter().map(|(k, v)| {
                    view! { cx,
                        <div class="p-4 bg-gray-100 rounded-lg hover:bg-gray-200">
                            <Icon stroke_width={2} stroke={"blue".to_string()} kind={ALARM_CHECK}/>
                        </div>
                        <div class="p-4 bg-gray-100 rounded-lg hover:bg-gray-200">
                            <Icon stroke_width={2} stroke={"red".to_string()} kind={X_CIRCLE}/>
                        </div>
                        <div class="p-4 bg-gray-100 rounded-lg hover:bg-gray-200">
                            <Icon stroke_width={2} stroke={"black".to_string()} kind={TRACTOR}/>
                        </div>
                    }
                // }).collect::<Vec<_>>()

            }
        </div>
    }
}

//  #[component]
// pub fn Counter( cx: Scope ) -> impl IntoView {
//
//     let (count, set_count) = create_signal(cx, 0 );
//
//     view! { cx,
//         <button
//             class="p-6 m-4 text-sm bg-gray-200 rounded-lg shadow-md hover:bg-gray-300 gap-2"
//             on:click=move |_| {
//                 set_count.set( count.get() + 1);
//             }
//         >
//             "Click me: "
//             {move || count.get()}
//         </button>
//     }
// }


