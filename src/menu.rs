use leptos::*;

use lepticons::*;

use crate::components::*;

#[component]
pub fn MainMenu(#[prop(default = "")] class: &'static str) -> impl IntoView {
    view! {
        <div class={format!("flex flex-row gap-4 {}", class)}>
            <a href="/">Icons</a>
            // <a href="/guide">Guide</a>
            // <a href="/packages">Packages</a>
            <a href="/license">License</a>
            <ThemeToggle/>
            <a href="https://github.com/eugener/lepticons"
               target={"_blank".to_string()}
               class="flex-none w-6 h-6  rounded-full bg-primary/100 text-secondary">
                <Icon glyph= move || LucideGlyph::Github
                      class="cursor-pointer p-[1px] pt-[3px] fill-secondary"/>
            </a>
        </div>
    }
}
