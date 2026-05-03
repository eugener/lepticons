use leptos::prelude::*;
use lepticons::{Icon, LucideGlyph};
use lepticons_animate::DrawIcon;
use lepticons_picker::{IconPicker, IconPickerPopover};

use crate::CommonHeader;

// Picker theme variables for `lepticons-picker` are defined in
// `demo/style/input.css` under `.lp-themed`, where light and dark variants
// are toggled via the `.dark` class on `<html>`.

const HERO_GLYPHS: [LucideGlyph; 6] = [
    LucideGlyph::Sparkles,
    LucideGlyph::Rocket,
    LucideGlyph::Heart,
    LucideGlyph::Compass,
    LucideGlyph::Star,
    LucideGlyph::Sun,
];

/// (label, hsl-string-for-`--highlight`)
const ACCENT_THEMES: [(&str, &str); 3] = [
    ("Sunset", "14 75% 43%"),
    ("Ocean", "210 75% 45%"),
    ("Violet", "265 60% 55%"),
];

#[component]
pub fn ComponentsView() -> impl IntoView {
    let (inline_selected, set_inline_selected) = signal(Some(LucideGlyph::Sparkles));
    let (popover_selected, set_popover_selected) = signal(Some(LucideGlyph::Rocket));
    let (accent_idx, set_accent_idx) = signal(0usize);
    let last_key = RwSignal::new(None::<&'static str>);

    // Live timing of one search (cached after first call to LucideGlyph::find)
    // so the page can quote a real number rather than a marketing adjective.
    let search_us = RwSignal::new(0u32);
    Effect::new(move |_| {
        let Some(window) = web_sys::window() else { return; };
        let Some(perf) = window.performance() else { return; };
        // Warm + measure
        let _ = LucideGlyph::find("");
        let start = perf.now();
        let _ = LucideGlyph::find("arrow up");
        let elapsed = perf.now() - start;
        search_us.set((elapsed * 1000.0).max(1.0) as u32);
    });

    // Cycle the hero glyph every 1.5s; new glyph re-runs the DrawIcon
    // animation because it's keyed on the index.
    let (hero_idx, set_hero_idx) = signal(0usize);
    Effect::new(move |_| {
        let i = hero_idx.get();
        set_timeout(
            move || set_hero_idx.set((i + 1) % HERO_GLYPHS.len()),
            std::time::Duration::from_millis(1500),
        );
    });

    let on_picker_keydown = move |ev: web_sys::KeyboardEvent| {
        let label = match ev.key().as_str() {
            "ArrowLeft" => Some("Left"),
            "ArrowRight" => Some("Right"),
            "ArrowUp" => Some("Up"),
            "ArrowDown" => Some("Down"),
            "Enter" => Some("Enter"),
            "Escape" => Some("Esc"),
            "/" => Some("Slash"),
            _ => None,
        };
        if let Some(l) = label {
            last_key.set(Some(l));
            set_timeout(
                move || {
                    if last_key.get_untracked() == Some(l) {
                        last_key.set(None);
                    }
                },
                std::time::Duration::from_millis(700),
            );
        }
    };

    let accent_style = move || {
        let (_, h) = ACCENT_THEMES[accent_idx.get()];
        format!("--highlight:{}", h)
    };

    view! {
        <div class="flex flex-col h-screen w-screen overflow-hidden">
            <div class="px-10 mt-5 flex-none">
                <CommonHeader/>
            </div>
            <div class="flex-1 min-h-0 overflow-y-auto px-10 -mt-6 pt-6">
                <Hero hero_idx=hero_idx/>
                <DemoSection
                    inline_selected=inline_selected
                    on_inline_select=Callback::new(move |g| set_inline_selected.set(Some(g)))
                    popover_selected=popover_selected
                    on_popover_select=Callback::new(move |g| set_popover_selected.set(Some(g)))
                    accent_idx=accent_idx
                    on_accent=Callback::new(move |i| set_accent_idx.set(i))
                    accent_style=Signal::derive(accent_style)
                    last_key=last_key
                    on_picker_keydown=Callback::new(on_picker_keydown)
                />
                <FeatureGrid/>
                <PerformanceLine us=search_us/>
                <CodeSnippet/>
                <CrossLinks/>
            </div>
        </div>
    }
}

// ----------------------------------------------------------------------------
// Hero
// ----------------------------------------------------------------------------

#[component]
fn Hero(hero_idx: ReadSignal<usize>) -> impl IntoView {
    let count = LucideGlyph::count();
    view! {
        <div class="flex flex-col items-center text-center mt-12 mb-12 px-4">
            <span class="px-3 py-1 mb-5 text-xs uppercase tracking-wider rounded-full
                         bg-primary/5 border border-primary/10 text-highlight">
                "lepticons-picker"
            </span>
            <div class="flex flex-row items-center gap-4 mb-2">
                <h1 class="text-3xl md:text-4xl lg:text-5xl font-semibold text-primary leading-tight max-w-3xl">
                    "The icon picker, fully assembled."
                </h1>
                <div class="hidden md:flex flex-none w-14 h-14 items-center justify-center
                            text-highlight rounded-md bg-primary/5 border border-primary/10">
                    {move || {
                        let g = HERO_GLYPHS[hero_idx.get()];
                        view! {
                            <DrawIcon glyph=g size="32" stroke_width="2" duration_ms=600/>
                        }
                    }}
                </div>
            </div>
            <p class="mt-3 text-base md:text-lg text-primary/70 max-w-2xl leading-relaxed">
                {format!(
                    "Search, browse, and copy {} Lucide icons with a drop-in Leptos \
                     component. Themable through CSS variables, fully keyboard-driven, \
                     persists recent picks to localStorage.",
                    count
                )}
            </p>
            <div class="flex flex-row flex-wrap items-center justify-center gap-2 mt-6">
                <CtaPrimary href="#try" label="Try it live" glyph=LucideGlyph::Sparkles/>
                <CtaSecondary href="#snippet" label="Quick start" glyph=LucideGlyph::Code/>
            </div>
        </div>
    }
}

#[component]
fn CtaPrimary(href: &'static str, label: &'static str, glyph: LucideGlyph) -> impl IntoView {
    view! {
        <a href=href
           class="flex flex-row items-center gap-2 px-4 py-2 rounded-md
                  bg-highlight text-white text-sm font-medium
                  hover:opacity-90 transition-opacity">
            <Icon glyph=glyph size="16"/>
            {label}
        </a>
    }
}

#[component]
fn CtaSecondary(href: &'static str, label: &'static str, glyph: LucideGlyph) -> impl IntoView {
    let external = href.starts_with("http");
    view! {
        <a href=href
           target=move || if external { "_blank" } else { "" }
           rel=move || if external { "noopener noreferrer" } else { "" }
           class="flex flex-row items-center gap-2 px-4 py-2 rounded-md
                  bg-primary/5 border border-primary/10 text-primary/80 text-sm
                  hover:border-highlight/50 hover:text-primary transition-colors">
            <Icon glyph=glyph size="16"/>
            {label}
        </a>
    }
}

// ----------------------------------------------------------------------------
// Demo section
// ----------------------------------------------------------------------------

#[component]
#[allow(clippy::too_many_arguments)]
fn DemoSection(
    inline_selected: ReadSignal<Option<LucideGlyph>>,
    on_inline_select: Callback<LucideGlyph>,
    popover_selected: ReadSignal<Option<LucideGlyph>>,
    on_popover_select: Callback<LucideGlyph>,
    accent_idx: ReadSignal<usize>,
    on_accent: Callback<usize>,
    accent_style: Signal<String>,
    last_key: RwSignal<Option<&'static str>>,
    on_picker_keydown: Callback<web_sys::KeyboardEvent>,
) -> impl IntoView {
    view! {
        <div id="try" class="flex flex-col gap-8 max-w-5xl w-full mx-auto px-4 mb-16">

            <DemoCard
                kicker="Inline"
                title="Embed it anywhere"
                description="Drop directly into a settings panel or editor surface.
                             Categories, search, recent-used strip, and copy-as-code
                             come bundled.">
                <ThemeSwatches accent_idx=accent_idx on_accent=on_accent/>
                <KeyboardIndicator last_key=last_key/>
                <div class="lp-themed w-full"
                     style=move || accent_style.get()
                     on:keydown=move |ev| on_picker_keydown.run(ev)>
                    <IconPicker
                        selected=inline_selected
                        on_select=on_inline_select
                        max_height="480px"
                    />
                </div>
                <SelectionReadout selected=inline_selected.into()/>
            </DemoCard>

            <DemoCard
                kicker="Popover"
                title="Or trigger from a button"
                description="Wrap any element. The picker pops with role=dialog,
                             closes on Escape or outside-click, and remembers
                             its resized width across opens.">
                <ProjectSettingsMockup
                    popover_selected=popover_selected
                    on_popover_select=on_popover_select
                />
                <SelectionReadout selected=popover_selected.into()/>
            </DemoCard>

        </div>
    }
}

#[component]
fn DemoCard(
    kicker: &'static str,
    title: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-5 p-6 bg-primary/5 rounded-lg border border-primary/10">
            <div>
                <div class="text-xs font-medium uppercase tracking-wider text-highlight mb-1">
                    {kicker}
                </div>
                <h2 class="text-xl font-semibold text-primary">{title}</h2>
                <p class="text-sm text-primary/60 mt-1 leading-relaxed">{description}</p>
            </div>
            {children()}
        </div>
    }
}

#[component]
fn ThemeSwatches(
    accent_idx: ReadSignal<usize>,
    on_accent: Callback<usize>,
) -> impl IntoView {
    view! {
        <div class="flex flex-row items-center gap-3 text-xs">
            <span class="text-primary/55 uppercase tracking-wider">"Theme"</span>
            <div class="flex flex-row gap-2">
                {ACCENT_THEMES.iter().enumerate().map(|(i, (label, hsl))| {
                    let label = *label;
                    let hsl = *hsl;
                    let bg = format!("background:hsl({})", hsl);
                    view! {
                        <button
                            type="button"
                            title=label
                            aria-label=label
                            on:click=move |_| on_accent.run(i)
                            class=move || {
                                let active = accent_idx.get() == i;
                                if active {
                                    "w-6 h-6 rounded-full ring-2 ring-offset-2 ring-offset-background ring-primary/40"
                                } else {
                                    "w-6 h-6 rounded-full opacity-70 hover:opacity-100 transition-opacity"
                                }
                            }
                            style=bg
                        ></button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn KeyboardIndicator(last_key: RwSignal<Option<&'static str>>) -> impl IntoView {
    let keys: [(&str, &str); 7] = [
        ("Left", "←"),
        ("Up", "↑"),
        ("Down", "↓"),
        ("Right", "→"),
        ("Enter", "Enter"),
        ("Slash", "/"),
        ("Esc", "Esc"),
    ];
    view! {
        <div class="flex flex-row items-center gap-3 text-xs">
            <span class="text-primary/55 uppercase tracking-wider">"Keys"</span>
            <div class="flex flex-row gap-1">
                {keys.iter().map(|(id, label)| {
                    let id = *id;
                    let label = *label;
                    let class_fn = move || {
                        let active = last_key.get() == Some(id);
                        if active {
                            "px-1.5 h-6 inline-flex items-center justify-center min-w-6
                             rounded text-[0.6875rem] font-mono leading-none
                             bg-highlight text-white border border-highlight
                             transition-colors"
                        } else {
                            "px-1.5 h-6 inline-flex items-center justify-center min-w-6
                             rounded text-[0.6875rem] font-mono leading-none
                             text-primary/55 bg-primary/5 border border-primary/10
                             transition-colors"
                        }
                    };
                    view! { <kbd class=class_fn>{label}</kbd> }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn ProjectSettingsMockup(
    popover_selected: ReadSignal<Option<LucideGlyph>>,
    on_popover_select: Callback<LucideGlyph>,
) -> impl IntoView {
    view! {
        <div class="lp-themed flex flex-col gap-5 p-5 rounded-md
                    bg-background border border-primary/10">
            <div class="text-xs uppercase tracking-wider text-primary/45 font-medium">
                "Project settings"
            </div>
            <div class="flex flex-row items-start gap-5">
                <div class="flex-none w-16 h-16 rounded-lg
                            bg-primary/10 text-highlight
                            flex items-center justify-center">
                    {move || popover_selected.get().map(|g| view! {
                        <Icon glyph=g size="32"/>
                    })}
                </div>
                <div class="flex flex-col gap-1.5 min-w-0">
                    <label class="text-sm font-medium text-primary">"Project icon"</label>
                    <IconPickerPopover
                        selected=popover_selected
                        on_select=on_popover_select
                        width="780px"
                        height="480px"
                        class="resize-x overflow-hidden min-w-[480px] max-w-[1100px]
                               border border-primary/10 bg-background rounded-lg"
                    >
                        <ProjectIconTrigger selected=popover_selected.into()/>
                    </IconPickerPopover>
                    <p class="text-xs text-primary/50">
                        "Used in the sidebar, browser tab, and the project switcher.
                         Drag the right edge of the popover to resize -- the size
                         is remembered across opens."
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProjectIconTrigger(selected: Signal<Option<LucideGlyph>>) -> impl IntoView {
    view! {
        <button class="flex items-center justify-between gap-3 px-3 py-2
                       w-full max-w-[28rem]
                       bg-primary/5 border border-primary/10 rounded-md
                       text-primary text-sm
                       hover:border-highlight/60 transition-colors">
            <div class="flex items-center gap-2.5 min-w-0">
                <div class="w-5 h-5 flex items-center justify-center text-highlight flex-none">
                    {move || selected.get().map(|g| view! { <Icon glyph=g size="16"/> })}
                </div>
                <span class="font-mono text-xs truncate">
                    {move || selected.get()
                        .map(|g| g.kebab_name())
                        .unwrap_or_else(|| "choose-icon".into())}
                </span>
            </div>
            <Icon glyph=LucideGlyph::ChevronDown size="14" class="text-primary/50 flex-none"/>
        </button>
    }
}

#[component]
fn SelectionReadout(selected: Signal<Option<LucideGlyph>>) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 pt-3 border-t border-primary/10 text-xs">
            <span class="uppercase tracking-wider text-primary/50">"Selected"</span>
            <span class="font-mono text-primary truncate">
                {move || selected.get()
                    .map(|g| format!("LucideGlyph::{}", g.name()))
                    .unwrap_or_else(|| "—".to_string())}
            </span>
        </div>
    }
}

// ----------------------------------------------------------------------------
// Feature grid (asymmetric: 3 large, 3 compact)
// ----------------------------------------------------------------------------

#[component]
fn FeatureGrid() -> impl IntoView {
    let primary: [(LucideGlyph, &'static str, &'static str); 3] = [
        (
            LucideGlyph::Search,
            "Searchable",
            "Match by name, tag, or category. Multi-word queries AND-merge for precision.",
        ),
        (
            LucideGlyph::Keyboard,
            "Keyboard-first",
            "Arrow keys move focus. Press / to jump to search, Enter to pick, Esc to close.",
        ),
        (
            LucideGlyph::Copy,
            "Copy as code",
            "One click copies LucideGlyph::Heart, the full <Icon /> tag, or raw SVG markup.",
        ),
    ];
    let secondary: [(LucideGlyph, &'static str, &'static str); 3] = [
        (
            LucideGlyph::History,
            "Recently used",
            "Last eight selections persist to localStorage; surfaced as a strip above the grid.",
        ),
        (
            LucideGlyph::Palette,
            "Themable",
            "Every color, radius, and tooltip is a CSS variable. Try the swatches above.",
        ),
        (
            LucideGlyph::Accessibility,
            "Accessible",
            "ARIA grid, gridcell, dialog, aria-selected. Roving tabindex on cells.",
        ),
    ];

    view! {
        <div class="max-w-5xl w-full mx-auto px-4 mb-10">
            <h2 class="text-2xl font-semibold text-primary mb-1">"Built in"</h2>
            <p class="text-sm text-primary/60 mb-6">
                "Every feature is a prop you can opt out of."
            </p>

            // Top row -- 3 primary features, larger
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                {primary.iter().map(|(glyph, title, desc)| {
                    let g = *glyph;
                    view! {
                        <div class="flex flex-col gap-3 p-5 rounded-lg
                                    bg-primary/5 border border-primary/10
                                    hover:border-highlight/40 transition-colors">
                            <div class="flex items-center justify-center w-10 h-10
                                        rounded-md bg-secondary text-highlight">
                                <Icon glyph=g size="22"/>
                            </div>
                            <h3 class="text-base font-semibold text-primary">{*title}</h3>
                            <p class="text-sm text-primary/60 leading-relaxed">{*desc}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Bottom row -- 3 supporting features, compact inline rows
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                {secondary.iter().map(|(glyph, title, desc)| {
                    let g = *glyph;
                    view! {
                        <div class="flex flex-row gap-3 items-start p-4 rounded-lg
                                    border border-primary/10
                                    hover:border-highlight/40 transition-colors">
                            <div class="flex-none w-8 h-8 flex items-center justify-center
                                        rounded-md bg-secondary text-highlight">
                                <Icon glyph=g size="16"/>
                            </div>
                            <div class="flex flex-col gap-0.5 min-w-0">
                                <h3 class="text-sm font-semibold text-primary">{*title}</h3>
                                <p class="text-xs text-primary/55 leading-relaxed">{*desc}</p>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn PerformanceLine(us: RwSignal<u32>) -> impl IntoView {
    let count = LucideGlyph::count();
    view! {
        <div class="max-w-5xl w-full mx-auto px-4 mb-16
                    text-center text-xs text-primary/50">
            {move || format!(
                "Searches all {} icons in ~{}\u{00B5}s on this device. Index built once, \
                 cached in OnceLock.",
                count,
                us.get().max(1)
            )}
        </div>
    }
}

// ----------------------------------------------------------------------------
// Code snippet with hand-rolled syntax highlighting
// ----------------------------------------------------------------------------

const SNIPPET: &str = r#"// Drop-in icon picker -- single dependency, no CSS imports.
use lepticons::LucideGlyph;
use lepticons_picker::IconPickerPopover;
use leptos::prelude::*;

#[component]
fn ProfileForm() -> impl IntoView {
    let (icon, set_icon) = signal(None::<LucideGlyph>);
    view! {
        <IconPickerPopover
            selected=icon
            on_select=Callback::new(move |g| set_icon.set(Some(g)))
        >
            <button>"Choose icon"</button>
        </IconPickerPopover>
    }
}"#;

#[derive(Clone, Copy)]
enum Tok {
    Default,
    Keyword,
    Macro,
    Attribute,
    Comment,
    Str,
}

impl Tok {
    fn class(self) -> &'static str {
        match self {
            Tok::Default => "text-primary/85",
            Tok::Keyword => "text-highlight",
            Tok::Macro => "text-highlight",
            Tok::Attribute => "text-primary/55 italic",
            Tok::Comment => "text-primary/40 italic",
            Tok::Str => "text-primary/65",
        }
    }
}

fn highlight_rust(src: &str) -> Vec<(String, Tok)> {
    let chars: Vec<char> = src.chars().collect();
    let mut out: Vec<(String, Tok)> = Vec::new();
    let mut buf = String::new();
    let mut i = 0;

    let push = |out: &mut Vec<(String, Tok)>, buf: &mut String, tok: Tok| {
        if !buf.is_empty() {
            out.push((std::mem::take(buf), tok));
        }
    };

    while i < chars.len() {
        let c = chars[i];

        // Line comment: //...
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            push(&mut out, &mut buf, Tok::Default);
            let mut s = String::new();
            while i < chars.len() && chars[i] != '\n' {
                s.push(chars[i]);
                i += 1;
            }
            out.push((s, Tok::Comment));
            continue;
        }

        // String literal "..."
        if c == '"' {
            push(&mut out, &mut buf, Tok::Default);
            let mut s = String::from('"');
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    s.push(chars[i]);
                    s.push(chars[i + 1]);
                    i += 2;
                    continue;
                }
                s.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                s.push('"');
                i += 1;
            }
            out.push((s, Tok::Str));
            continue;
        }

        // Attribute #[...]
        if c == '#' && i + 1 < chars.len() && chars[i + 1] == '[' {
            push(&mut out, &mut buf, Tok::Default);
            let mut s = String::new();
            while i < chars.len() {
                s.push(chars[i]);
                if chars[i] == ']' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push((s, Tok::Attribute));
            continue;
        }

        // Identifier / keyword / macro
        if c.is_alphabetic() || c == '_' {
            push(&mut out, &mut buf, Tok::Default);
            let mut id = String::new();
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                id.push(chars[i]);
                i += 1;
            }
            // macro?
            if i < chars.len() && chars[i] == '!' {
                id.push('!');
                i += 1;
                out.push((id, Tok::Macro));
                continue;
            }
            let tok = match id.as_str() {
                "use" | "fn" | "let" | "mut" | "move" | "impl" | "struct" | "enum"
                | "match" | "if" | "else" | "for" | "while" | "loop" | "return"
                | "as" | "in" | "pub" | "mod" | "self" | "true" | "false" | "Some"
                | "None" => Tok::Keyword,
                _ => Tok::Default,
            };
            out.push((id, tok));
            continue;
        }

        buf.push(c);
        i += 1;
    }
    push(&mut out, &mut buf, Tok::Default);
    out
}

#[component]
fn CodeSnippet() -> impl IntoView {
    let tokens = highlight_rust(SNIPPET);
    view! {
        <div id="snippet" class="max-w-4xl w-full mx-auto px-4 mb-16">
            <h2 class="text-2xl font-semibold text-primary mb-1">
                "Eight lines from zero to a picker"
            </h2>
            <p class="text-sm text-primary/60 mb-5">
                "Single dependency. No CSS imports. Works in any Leptos 0.8 app."
            </p>
            <pre class="p-5 rounded-lg bg-secondary border border-primary/10
                        overflow-x-auto text-xs leading-relaxed font-mono">
                <code>
                    {tokens.into_iter().map(|(text, tok)| {
                        view! { <span class=tok.class()>{text}</span> }
                    }).collect::<Vec<_>>()}
                </code>
            </pre>
            <div class="mt-3 text-xs text-primary/50 font-mono">
                "cargo add lepticons lepticons-picker"
            </div>
        </div>
    }
}

// ----------------------------------------------------------------------------
// Cross-links footer
// ----------------------------------------------------------------------------

#[component]
fn CrossLinks() -> impl IntoView {
    let count = LucideGlyph::count();
    view! {
        <div class="max-w-4xl w-full mx-auto px-4 pb-16 text-center">
            <a href="/" class="text-sm text-highlight hover:underline">
                {format!("← Browse all {} icons", count)}
            </a>
        </div>
    }
}
