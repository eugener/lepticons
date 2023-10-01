use leptos::logging::*;
use leptos::*;
use leptos_meta::*;
use lucide_icons::*;

use crate::local_storage::LocalStorage;

#[component]
pub fn StickyTop(#[prop(default = "")] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class={format!("sticky top-0 z-50 {}", class)}>
            {children()}
        </div>
    }
}

const DARK_MODE: &str = "dark-mode";

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (dark_mode, set_prefers_dark) =
        create_signal(LocalStorage::get(DARK_MODE).unwrap_or_default());

    create_effect(move |_| {
        LocalStorage::set(DARK_MODE, &dark_mode.get());
    });

    let theme = move || {
        log!(">>> Theme: {}", dark_mode.get());
        if dark_mode.get() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    let pos_class = move || {
        format!(
            "flex flex-row gap-2 items-center {} w-12 h-6 bg-primary/50 rounded-full",
            if dark_mode.get() {
                "justify-start".to_string()
            } else {
                "justify-end".to_string()
            }
        )
    };

    let toggle_theme = move |_| {
        set_prefers_dark.update(|dark| *dark = !*dark);
    };

    let theme_glyph = move || {
        if dark_mode.get() {
            LucideGlyph::Moon
        } else {
            LucideGlyph::Sun
        }
    };

    view! {
        <Html class=theme />
        <button class=pos_class
             on:click=toggle_theme >
            <div class="flex-none w-6 h-6 bg-primary/100 rounded-full">
                <Icon<LucideGlyph> glyph=theme_glyph class="text-secondary p-1"/>
            </div>
        </button>
    }
}
