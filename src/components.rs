use leptos::*;

#[component]
pub fn StickyTop(
    #[prop(default = "")]
    class: &'static str,
    children: Children
) -> impl IntoView {
    view! {
        <div class={format!("sticky top-0 z-50 {}", class)}>
            {children()}
        </div>
    }
}