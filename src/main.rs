use leptos::*;
use lucid_icons_leptos::ProgressBar;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {

    let (count, _set_count) = create_signal(cx, 5);

    // view! { cx,
    //     <button
    //         on:click=move |_| {
    //             set_count.set( count.get() + 1);
    //         }
    //     >
    //         "Click me: "
    //         {move || count.get()}
    //     </button>
    //     // <ProgressBar progress=count/>
    // }

    view! { cx,
            <Counter/>
            <Counter/>
            <Counter/>
            <ProgressBar progress={count}/>
    }
}

 #[component]
pub fn Counter( cx: Scope ) -> impl IntoView {

    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <button
            on:click=move |_| {
                set_count.set( count.get() + 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}



// fn Icon() -> impl leptos::IntoView {
//     // view! {
//     //     <svg
//     //         width="24"
//     //         height="24"
//     //         viewBox="0 0 24 24"
//     //         fill="none"
//     //         stroke="currentColor"
//     //         stroke-width="2"
//     //         stroke-linecap="round"
//     //         stroke-linejoin="round"
//     //     >
//     //         <circle cx="12" cy="12" r="10"></circle>
//     //         <line x1="12" y1="8" x2="12" y2="12"></line>
//     //         <line x1="12" y1="16" x2="12" y2="16"></line>
//     //     </svg>
//     // }

//     let mut svg = leptos::svg::svg();
//     leptos::IntoView::into_view(svg)
// }
