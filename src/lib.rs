use leptos::*;
// use styled::style;

 #[component]
pub fn ProgressBar(
    cx: Scope,
//     // mark this prop optional
//     // you can specify it or not when you use <ProgressBar/>
    #[prop(optional)] _max: f64,
    progress: ReadSignal<i32>
) -> impl IntoView {
    view! { cx,
        <progress
            max=10
            value=progress.get()
        />
    }
}


#[component]
pub fn LucidIcon(cx: Scope) -> impl IntoView {
    view! { cx,
        <img src="svg/chevrons-right.svg" />
    }
}
