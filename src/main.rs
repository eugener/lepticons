use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use icons_view::*;

mod components;
mod icons_view;
mod local_storage;

//TODO show and select icon attributes
//     show and select categories
//     main menu

fn main() {
    provide_meta_context();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=IconsView/>
                <Route path="/*any" view=NotFoundView/>
            </Routes>
        </Router>
    }
}

#[component]
fn NotFoundView() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-screen">
            <div class="text-4xl font-medium">404</div>
            <div class="text-2xl font-medium">Page not found</div>
        </div>
    }
}
