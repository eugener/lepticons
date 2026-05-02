use chrono::Datelike;
use chrono::Utc;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

use components_view::*;
use icons_view::*;

use crate::components::DarkMode;
use crate::menu::*;

mod components;
mod components_view;
mod icons_view;
mod local_storage;
mod menu;

fn main() {
    provide_meta_context();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    DarkMode::provide();

    view! {
        <Router>
            <Routes fallback=|| view! { <NotFoundView/> }>
                <Route path=path!("/") view=IconsView/>
                <Route path=path!("/icons/:name") view=IconPermalinkView/>
                <Route path=path!("/components") view=ComponentsView/>
                <Route path=path!("/license") view=LicenseView/>
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

#[component]
pub fn CommonHeader() -> impl IntoView {
    // items-start keeps MainMenu top-aligned to match the home page (where the
    // menu sits at `mt-5` of the right pane). Bottom-aligning the menu against
    // the taller logo+version block makes navigation visually jump between pages.
    view! {
        <div class="flex flex-row items-start">
            <div class="flex flex-col pr-4">
                <img src="lepticons.png" class="w-48"/>
                <div class="self-end pr-2 -mt-1">
                    <VersionLink/>
                </div>
            </div>
            <MainMenu class="flex-auto justify-end text-primary"/>
        </div>
        <hr/>
    }
}

const COPYRIGHT: [&str; 4] = [

    "Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT).
    All other copyright (c) for Lucide are held by Lucide Contributors 2022.",

    "Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the \"Software\"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:",

    "The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.",

    "THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE."

];

#[component]
fn LicenseView() -> impl IntoView {
    let year = Utc::now().year().to_string();
    let copyright = format!("Copyright (c) 2022-{} Eugene Ryzhikov", year);

    view! {
            <div class="flex flex-col h-screen w-screen py-5 px-10 overflow-y-auto">
                <CommonHeader/>
                <div class="mx-40">

                <p class="my-10  p-25 text-2xl text-primary">MIT License</p>
                <p class="text-primary py-5">
                    {copyright}
                </p>
         {
             COPYRIGHT.iter().map(|c| view! {
                <p class="text-primary py-1">{c.to_string()}</p>
            }).collect::<Vec<_>>()
         }
        </div>
    </div>
    }
}
