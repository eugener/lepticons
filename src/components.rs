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

    // let theme_glyph = move || {
    //     if prefers_dark.get() {
    //         LucideGlyph::Moon
    //     } else {
    //         LucideGlyph::Sun
    //     }
    // };

    view! {
        <Html class=theme />
        <button class=pos_class
             on:click=toggle_theme >
            <div class="flex-none w-6 h-6 bg-primary/100 rounded-full">
                // <Icon glyph= MaybeSignal::from( move || theme_glyph)
                //       class="text-secondary p-1" size={24}/>
                      // class="text-secondary p-1" size={24}/>
                {
                    move || if prefers_dark.get() {
                        view!{<Icon glyph=LucideGlyph::Moon
                                    class="text-secondary p-1"
                                    size={24} />}
                    } else {
                        view!{<Icon glyph=LucideGlyph::Sun
                                    class="text-secondary p-1"
                                    size={24} />}
                    }
                }
            </div>
        </button>
    }
}
