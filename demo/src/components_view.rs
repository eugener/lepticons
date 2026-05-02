use leptos::prelude::*;
use lepticons::{Icon, LucideGlyph};
use lepticons_picker::{IconPicker, IconPickerPopover};

use crate::components::StickyTop;
use crate::menu::*;

// Picker theme variables and the off-white `bg-soft` accent are defined in
// `demo/style/input.css` under `.lp-themed` and `.bg-soft`, where light and
// dark variants are toggled via the `.dark` class on `<html>`.

const NAV: [(&str, &str); 4] = [
    ("inline", "Inline picker"),
    ("popover", "Popover picker"),
    ("features", "Features"),
    ("integrate", "Integration"),
];

#[component]
pub fn ComponentsView() -> impl IntoView {
    let (inline_selected, set_inline_selected) = signal(Some(LucideGlyph::Sparkles));
    let (popover_selected, set_popover_selected) = signal(Some(LucideGlyph::Rocket));

    view! {
        <div class="flex flex-row">
            <Sidebar/>
            <div class="px-10 mt-5 flex flex-col flex-auto h-screen overflow-y-auto overflow-x-hidden">
                <StickyTop class="bg-gradient-to-b from-85% from-background to-100% to-transparent">
                    <MainMenu class="justify-end text-primary"/>
                </StickyTop>
                <Hero/>
                <InlineSection
                    selected=inline_selected
                    set_selected=set_inline_selected
                />
                <SectionDivider/>
                <PopoverSection
                    selected=popover_selected
                    set_selected=set_popover_selected
                />
                <SectionDivider/>
                <FeaturesSection/>
                <SectionDivider/>
                <CodeSection/>
                <Footer/>
            </div>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <div class="w-64 flex-none bg-secondary h-screen overflow-y-auto">
            <StickyTop class="px-10 bg-gradient-to-b from-95% from-secondary to-100% to-transparent">
                <a href="/" class="flex flex-col items-center gap-0">
                    <img src="lepticons.png" class="pt-5 w-48"/>
                    <div class="flex flex-row w-full justify-between pb-2 text-primary text-xs items-center">
                        <span>"Components"</span>
                        <VersionLink/>
                    </div>
                </a>
                <hr/>
            </StickyTop>

            <div class="px-10 pt-5 flex flex-col gap-2">
                {NAV.iter().map(|(anchor, label)| view! {
                    <a href=format!("#{}", anchor)
                       class="text-sm text-primary/70 hover:text-highlight transition-colors">
                        {*label}
                    </a>
                }).collect::<Vec<_>>()}
            </div>

            <div class="mx-10 mt-8 p-4 flex flex-col gap-2 bg-primary/5 rounded-lg
                        border border-primary/10 text-xs text-primary/60 leading-relaxed">
                <div class="font-semibold text-primary text-[0.6875rem] uppercase tracking-wider">
                    "Crate"
                </div>
                <div class="font-mono text-primary/80">
                    "lepticons-picker"
                </div>
                <div>
                    "Drop-in icon picker for Leptos. Search, MRU, copy-as-code."
                </div>
                <a href="https://crates.io/crates/lepticons-picker"
                   target="_blank"
                   class="text-highlight hover:underline">
                    "View on crates.io →"
                </a>
            </div>
        </div>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let count = LucideGlyph::count();
    view! {
        <div class="pt-10 pb-8 max-w-3xl">
            <div class="text-xs uppercase tracking-wider text-highlight mb-2">
                "lepticons-picker"
            </div>
            <h1 class="text-4xl font-semibold text-primary leading-tight">
                "The icon picker, fully assembled."
            </h1>
            <p class="mt-4 text-base text-primary/70 leading-relaxed">
                {format!(
                    "Search, browse, and copy {} Lucide icons with a drop-in Leptos \
                     component. Themable through CSS variables, fully keyboard-driven, \
                     persists recent picks to localStorage.",
                    count
                )}
            </p>
        </div>
    }
}

#[component]
fn SectionDivider() -> impl IntoView {
    view! { <hr class="my-12 border-primary/10"/> }
}

#[component]
fn SectionHeader(
    anchor: &'static str,
    kicker: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div id=anchor class="mb-6 max-w-2xl scroll-mt-20">
            <div class="text-xs uppercase tracking-wider text-highlight mb-1">
                {kicker}
            </div>
            <h2 class="text-2xl font-semibold text-primary">{title}</h2>
            <p class="mt-2 text-sm text-primary/60 leading-relaxed">{description}</p>
        </div>
    }
}

#[component]
fn InlineSection(
    selected: ReadSignal<Option<LucideGlyph>>,
    set_selected: WriteSignal<Option<LucideGlyph>>,
) -> impl IntoView {
    view! {
        <SectionHeader
            anchor="inline"
            kicker="Inline"
            title="Embed it anywhere"
            description="Drop directly into a settings panel or editor surface.
                         Categories, search, recent-used strip, and copy-as-code
                         come bundled."
        />
        <div class="lp-themed w-full max-w-3xl">
            <IconPicker
                selected=selected
                on_select=Callback::new(move |g| set_selected.set(Some(g)))
                max_height="480px"
            />
        </div>
        <SelectionReadout selected=selected.into() />
    }
}

#[component]
fn PopoverSection(
    selected: ReadSignal<Option<LucideGlyph>>,
    set_selected: WriteSignal<Option<LucideGlyph>>,
) -> impl IntoView {
    view! {
        <SectionHeader
            anchor="popover"
            kicker="Popover"
            title="Or trigger from a button"
            description="Wrap any element. The picker pops with role=dialog,
                         closes on Escape or outside-click, remembers its
                         resized width across opens."
        />
        <div class="lp-themed flex flex-row flex-wrap items-start gap-6">
            <IconPickerPopover
                selected=selected
                on_select=Callback::new(move |g| set_selected.set(Some(g)))
                width="780px"
                height="480px"
                class="resize-x overflow-hidden min-w-[480px] max-w-[1100px]
                       border border-border bg-card rounded-lg"
            >
                <PopoverTrigger selected=selected.into()/>
            </IconPickerPopover>
            <p class="text-sm text-primary/55 max-w-xs pt-2">
                "Click the button to open the picker. Drag the right edge of
                 the popover to resize -- the size is remembered across opens."
            </p>
        </div>
        <SelectionReadout selected=selected.into() />
    }
}

#[component]
fn PopoverTrigger(selected: Signal<Option<LucideGlyph>>) -> impl IntoView {
    view! {
        <button class="flex items-center justify-between gap-3 px-4 py-2.5
                       w-[780px] max-w-full
                       bg-secondary border border-border rounded-md
                       text-primary hover:border-highlight/60 transition-colors">
            <div class="flex items-center gap-3 min-w-0">
                <div class="w-6 h-6 flex items-center justify-center text-highlight flex-none">
                    {move || selected.get().map(|g| view! { <Icon glyph=g size="20" /> })}
                </div>
                <span class="text-sm font-medium truncate">
                    {move || selected.get().map(|g| g.kebab_name()).unwrap_or_else(|| "Choose icon".into())}
                </span>
            </div>
            <Icon glyph=LucideGlyph::ChevronDown size="16" class="text-primary/50 flex-none" />
        </button>
    }
}

#[component]
fn SelectionReadout(selected: Signal<Option<LucideGlyph>>) -> impl IntoView {
    view! {
        <div class="flex items-center gap-4 mt-5 pt-4 border-t border-primary/10 max-w-2xl">
            <div class="w-12 h-12 flex items-center justify-center rounded-md
                        bg-secondary text-highlight">
                {move || selected.get().map(|g| view! { <Icon glyph=g size="28" /> })}
            </div>
            <div class="flex flex-col min-w-0">
                <span class="text-[0.6875rem] uppercase tracking-wider text-primary/50">
                    "Selected"
                </span>
                <span class="text-sm font-mono text-primary truncate">
                    {move || selected.get()
                        .map(|g| format!("LucideGlyph::{}", g.name()))
                        .unwrap_or_else(|| "—".to_string())}
                </span>
            </div>
        </div>
    }
}

#[component]
fn FeaturesSection() -> impl IntoView {
    let features: [(LucideGlyph, &'static str, &'static str); 6] = [
        (LucideGlyph::Search, "Searchable",
            "Match icons by name, tag, or category. Multi-word queries AND-merge for precision."),
        (LucideGlyph::Keyboard, "Keyboard-first",
            "Arrow keys navigate the grid. Press / to focus search, Enter to pick, Esc to close."),
        (LucideGlyph::History, "Recently used",
            "The last eight selections persist to localStorage and surface as a strip above the grid."),
        (LucideGlyph::Copy, "Copy as code",
            "One click copies LucideGlyph::Heart, the full <Icon /> tag, or raw SVG markup."),
        (LucideGlyph::Palette, "Themable",
            "All colors, radii, and tooltips are CSS variables. This page maps them to the demo's tokens."),
        (LucideGlyph::Accessibility, "Accessible",
            "ARIA grid, gridcell, dialog, and aria-selected wired up. Roving tabindex on cells."),
    ];

    view! {
        <SectionHeader
            anchor="features"
            kicker="Built-in"
            title="Everything you need"
            description="Every feature is a prop you can opt out of."
        />
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-x-8 gap-y-6 max-w-5xl">
            {features.iter().map(|(glyph, title, desc)| {
                let g = *glyph;
                view! {
                    <div class="flex flex-row gap-3 items-start">
                        <div class="flex-none w-9 h-9 flex items-center justify-center
                                    rounded-md bg-secondary text-highlight">
                            <Icon glyph=g size="18" />
                        </div>
                        <div class="flex flex-col gap-1 min-w-0">
                            <h3 class="text-sm font-semibold text-primary">{*title}</h3>
                            <p class="text-xs text-primary/60 leading-relaxed">{*desc}</p>
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

const SNIPPET: &str = r#"use lepticons::LucideGlyph;
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

#[component]
fn CodeSection() -> impl IntoView {
    view! {
        <SectionHeader
            anchor="integrate"
            kicker="Integrate"
            title="Eight lines from zero to a picker"
            description="Single dependency. No CSS imports. Works in any Leptos 0.8 app."
        />
        <pre class="p-5 rounded-lg bg-secondary border border-primary/10
                    overflow-x-auto text-xs leading-relaxed
                    font-mono text-primary/85 max-w-4xl">
            <code>{SNIPPET}</code>
        </pre>
        <div class="mt-3 text-xs text-primary/50 font-mono">
            "cargo add lepticons lepticons-picker"
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let count = LucideGlyph::count();
    view! {
        <div class="pt-16 pb-12">
            <a href="/" class="text-sm text-highlight hover:underline">
                {format!("← Browse all {} icons", count)}
            </a>
        </div>
    }
}
