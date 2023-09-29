use leptos::*;
use leptos_meta::*;
use lucide_icons::*;

#[component]
pub fn StickyTop(#[prop(default = "")] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class={format!("sticky top-0 z-50 {}", class)}>
            {children()}
        </div>
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (prefers_dark, set_prefers_dark) = create_signal(false);

    let theme = move || {
        if prefers_dark.get() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    // let glyph = move || {
    //     as_glyph(prefers_dark.get())
    //     // let result: dyn Glyph = if prefers_dark.get() {
    //     //     LucideGlyph::Moon as dyn Glyph
    //     // } else {
    //     //     LucideGlyph::Sun
    //     // };
    //     // result
    // };
    //
    // fn as_glyph <T: Glyph + 'static>( dark_mode: bool ) -> T {
    //      if dark_mode {
    //         LucideGlyph::Moon
    //     } else {
    //         LucideGlyph::Sun
    //     }
    // }

    let pos_class = move || {
        format!(
            "flex flex-row gap-2 items-center {} w-12 h-6 bg-primary/50 rounded-full",
            if prefers_dark.get() {
                "justify-start".to_string()
            } else {
                "justify-end".to_string()
            }
        )
    };

    let toggle_theme = move |_| {
        set_prefers_dark.set(!prefers_dark.get());
    };

    view! {
        <Html class=theme />
        <button class=pos_class
             on:click=toggle_theme >
            <div class="flex-none w-6 h-6 bg-primary/100 rounded-full"  >
                {
                    if prefers_dark.get() {
                        view!{<Icon glyph=LucideGlyph::Sun class="text-primary p-1" size={24}/>}
                    } else {
                        view!{<Icon glyph=LucideGlyph::Moon class="text-secondary p-1" size={24}/>}
                    }
                }
            </div>
        </button>
    }
}
