use chrono::Datelike;
use chrono::Utc;
use leptos::ev::keydown;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

use components_view::*;
use icons_view::*;

use crate::components::{DarkMode, HelpOpen, KeyboardHelp};
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
    let help = HelpOpen::provide();

    // Global window keydown for `?` (toggle the shortcut overlay) and Esc
    // (close it when open). Skipped while the user is typing in an input.
    window_event_listener(keydown, move |ev: web_sys::KeyboardEvent| {
        let in_input = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            .map(|el| {
                let tag = el.tag_name();
                tag.eq_ignore_ascii_case("input") || tag.eq_ignore_ascii_case("textarea")
            })
            .unwrap_or(false);
        match ev.key().as_str() {
            "?" if !in_input => {
                ev.prevent_default();
                help.write.update(|b| *b = !*b);
            }
            "Escape" if help.read.get_untracked() => {
                help.write.set(false);
            }
            _ => {}
        }
    });

    view! {
        <Router>
            <Routes fallback=|| view! { <NotFoundView/> }>
                <Route path=path!("/") view=IconsView/>
                <Route path=path!("/icons/:name") view=IconPermalinkView/>
                <Route path=path!("/components") view=ComponentsView/>
                <Route path=path!("/license") view=LicenseView/>
            </Routes>
        </Router>
        {move || help.read.get().then(|| view! {
            <KeyboardHelp on_close=Callback::new(move |_| help.write.set(false))/>
        })}
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
    view! {
        <div class="flex flex-col">
            <div class="flex flex-row items-center gap-4 pb-4 border-b border-primary/10">
                <a href="/" class="flex-none">
                    <img src="lepticons.png" class="h-9 w-auto" alt="Lepticons"/>
                </a>
                <MainMenu class="flex-auto justify-end text-primary"/>
            </div>
            <div class="h-6 bg-gradient-to-b from-background to-transparent pointer-events-none"></div>
        </div>
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
        <div class="flex flex-col h-screen w-screen overflow-hidden">
            <div class="px-10 mt-5 flex-none">
                <CommonHeader/>
            </div>
            <div class="flex-1 min-h-0 overflow-y-auto px-10 pb-10 -mt-6 pt-6">
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
        </div>
    }
}
