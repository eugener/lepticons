use leptos::prelude::*;
use lepticons::{CustomIcon, Icon, LucideGlyph};

use crate::components::*;

/// Small "vX.Y.Z" link to crates.io with a styled hover tooltip listing
/// all three crate versions on separate lines.
#[component]
pub fn VersionLink() -> impl IntoView {
    view! {
        <div class="relative group">
            <a href="https://crates.io/crates/lepticons"
               target={"_blank".to_string()}
               rel={"noopener noreferrer".to_string()}
               class="text-primary text-xs opacity-60 hover:opacity-100">
                {format!("v{}", lepticons::VERSION)}
            </a>
            <div class="hidden group-hover:flex absolute right-0 top-full mt-2 z-50 flex-col gap-0.5 bg-secondary border border-primary/20 rounded-md px-3 py-2 text-xs text-primary whitespace-nowrap shadow-lg">
                <div class="flex flex-row justify-between gap-6">
                    <span class="opacity-60">"lepticons"</span>
                    <span>{lepticons::VERSION}</span>
                </div>
                <div class="flex flex-row justify-between gap-6">
                    <span class="opacity-60">"lepticons-picker"</span>
                    <span>{lepticons_picker::VERSION}</span>
                </div>
                <div class="flex flex-row justify-between gap-6">
                    <span class="opacity-60">"lepticons-animate"</span>
                    <span>{lepticons_animate::VERSION}</span>
                </div>
            </div>
        </div>
    }
}

// Lucide dropped brand logos in v1 (trademark constraints, design-consistency rules);
// see https://github.com/lucide-icons/lucide/blob/main/BRAND_LOGOS_STATEMENT.md.
// Inline SVG sourced from Simple Icons, Lucide's recommended brand-logo alternative.
const GITHUB_SVG: &str = r#"<path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>"#;

/// Base wrapper class for a 24px menu icon (no color/opacity treatment).
const MENU_ICON_BASE: &str =
    "flex flex-none items-center justify-center w-6 h-6 cursor-pointer";

/// Wrapper class for menu icons that should follow the muted/hover treatment.
const MENU_ICON_LINK: &str =
    "flex flex-none items-center justify-center w-6 h-6 cursor-pointer \
     text-primary opacity-80 hover:opacity-100";

/// External anchor wrapping a Lucide glyph, styled like the rest of the menu icons.
#[component]
fn MenuIconLink(
    href: &'static str,
    title: &'static str,
    glyph: LucideGlyph,
) -> impl IntoView {
    view! {
        <a href=href
           target={"_blank".to_string()}
           rel={"noopener noreferrer".to_string()}
           class=MENU_ICON_LINK
           title=title
           aria-label=title>
            <Icon glyph=glyph size="24" stroke_width="2" class="block" />
        </a>
    }
}

#[component]
pub fn MainMenu(#[prop(default = "")] class: &'static str) -> impl IntoView {
    let help = use_context::<HelpOpen>();
    view! {
        <div class={format!("flex flex-row gap-4 items-center {}", class)}>
            <a href="/" class="leading-none">"Icons"</a>
            <a href="/components" class="leading-none">"Components"</a>
            <a href="/license" class="leading-none">"License"</a>
            <ThemeToggle/>
            {help.map(|h| view! {
                <button
                    class=MENU_ICON_LINK
                    title="Keyboard shortcuts (?)"
                    aria-label="Open keyboard shortcuts"
                    on:click=move |_| h.write.update(|b| *b = !*b)
                >
                    <Icon glyph=LucideGlyph::CircleQuestionMark size="24" stroke_width="2" class="block" />
                </button>
            })}
            <div class="flex flex-row gap-1.5 items-center">
                <a href="https://github.com/eugener/lepticons"
                   target={"_blank".to_string()}
                   rel={"noopener noreferrer".to_string()}
                   class=MENU_ICON_BASE
                   title="GitHub"
                   aria-label="GitHub">
                    <CustomIcon
                        svg=GITHUB_SVG
                        class="w-6 h-6 fill-primary block"
                        fill="currentColor"
                        stroke="none"
                    />
                </a>
                <MenuIconLink
                    href="https://crates.io/crates/lepticons"
                    title="lepticons on crates.io"
                    glyph=LucideGlyph::Package
                />
                <MenuIconLink
                    href="https://docs.rs/lepticons"
                    title="API docs on docs.rs"
                    glyph=LucideGlyph::BookOpen
                />
            </div>
        </div>
    }
}
