use leptos::prelude::*;
use lepticons::{Icon, LucideGlyph};
use lepticons_picker::{IconPicker, IconPickerPopover};

use crate::CommonHeader;

// Picker theme variables and the off-white `bg-soft` accent are defined in
// `demo/style/input.css` under `.lp-themed` and `.bg-soft`, where light and
// dark variants are toggled via the `.dark` class on `<html>`.

#[component]
pub fn ComponentsView() -> impl IntoView {
    let (inline_selected, set_inline_selected) = signal(Some(LucideGlyph::Sparkles));
    let (popover_selected, set_popover_selected) = signal(Some(LucideGlyph::Rocket));

    view! {
        <div class="flex flex-col h-screen w-screen py-5 px-10 overflow-y-auto">
            <CommonHeader/>
            <Hero/>
            <DemoRow
                inline_selected=inline_selected
                set_inline_selected=set_inline_selected
                popover_selected=popover_selected
                set_popover_selected=set_popover_selected
            />
            <FeatureGrid/>
            <CodeSnippet/>
            <Footer/>
        </div>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let count = LucideGlyph::count();
    view! {
        <div class="flex flex-col items-center text-center mt-16 mb-12 px-4">
            <span class="px-3 py-1 mb-5 text-xs uppercase tracking-wider rounded-full
                         border border-border text-highlight bg-soft">
                "lepticons-picker"
            </span>
            <h1 class="text-4xl md:text-5xl font-semibold text-primary leading-tight max-w-3xl">
                "The icon picker, fully assembled."
            </h1>
            <p class="mt-5 text-lg text-primary/70 max-w-2xl leading-relaxed">
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
fn DemoRow(
    inline_selected: ReadSignal<Option<LucideGlyph>>,
    set_inline_selected: WriteSignal<Option<LucideGlyph>>,
    popover_selected: ReadSignal<Option<LucideGlyph>>,
    set_popover_selected: WriteSignal<Option<LucideGlyph>>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 max-w-5xl w-full mx-auto px-4 mb-16">
            <DemoCard
                kicker="Inline"
                title="Embed it anywhere"
                description="Drop directly into a settings panel or editor surface.
                             Categories, search, recent-used strip, and copy-as-code
                             come bundled.">
                <div class="lp-themed w-full">
                    <IconPicker
                        selected=inline_selected
                        on_select=Callback::new(move |g| set_inline_selected.set(Some(g)))
                        max_height="480px"
                    />
                </div>
                <SelectionReadout selected=inline_selected.into() />
            </DemoCard>

            <DemoCard
                kicker="Popover"
                title="Or trigger from a button"
                description="Wrap any element. The picker pops with role=dialog,
                             closes on Escape or outside-click, and snaps back
                             into the page flow.">
                <div class="lp-themed flex flex-row flex-wrap items-center gap-6">
                    <IconPickerPopover
                        selected=popover_selected
                        on_select=Callback::new(move |g| set_popover_selected.set(Some(g)))
                        width="540px"
                        height="440px"
                    >
                        <PopoverTrigger selected=popover_selected.into()/>
                    </IconPickerPopover>
                    <p class="text-sm text-primary/55 max-w-md">
                        "Click the button to open the picker. It mounts in place,
                         supports the full keyboard nav, and dismisses on Escape
                         or click-outside."
                    </p>
                </div>
                <SelectionReadout selected=popover_selected.into() />
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
        <div class="flex flex-col gap-5 p-6 bg-card border border-border rounded-lg
                    shadow-sm hover:shadow-md transition-shadow">
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
fn PopoverTrigger(selected: Signal<Option<LucideGlyph>>) -> impl IntoView {
    view! {
        <button class="flex items-center gap-3 px-4 py-2.5
                       bg-secondary border border-border rounded-md
                       text-primary hover:border-highlight/60 transition-colors">
            <div class="w-6 h-6 flex items-center justify-center text-highlight">
                {move || selected.get().map(|g| view! { <Icon glyph=g size="20" /> })}
            </div>
            <span class="text-sm font-medium">
                {move || selected.get().map(|g| g.kebab_name()).unwrap_or_else(|| "Choose icon".into())}
            </span>
            <Icon glyph=LucideGlyph::ChevronDown size="16" class="text-primary/50" />
        </button>
    }
}

#[component]
fn SelectionReadout(selected: Signal<Option<LucideGlyph>>) -> impl IntoView {
    view! {
        <div class="flex items-center gap-4 pt-3 border-t border-border">
            <div class="w-14 h-14 flex items-center justify-center rounded-md
                        bg-secondary text-highlight">
                {move || selected.get().map(|g| view! { <Icon glyph=g size="32" /> })}
            </div>
            <div class="flex flex-col min-w-0">
                <span class="text-xs uppercase tracking-wider text-primary/50">
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
fn FeatureGrid() -> impl IntoView {
    let features: [(LucideGlyph, &'static str, &'static str); 6] = [
        (
            LucideGlyph::Search,
            "Searchable",
            "Match icons by name, tag, or category. Multi-word queries AND-merge for precision.",
        ),
        (
            LucideGlyph::Keyboard,
            "Keyboard-first",
            "Arrow keys navigate the grid. Press / to focus search, Enter to pick, Esc to close.",
        ),
        (
            LucideGlyph::History,
            "Recently used",
            "The last eight selections are persisted to localStorage and surfaced as a strip above the grid.",
        ),
        (
            LucideGlyph::Copy,
            "Copy as code",
            "One click copies LucideGlyph::Heart, the full <Icon /> tag, or raw SVG markup.",
        ),
        (
            LucideGlyph::Palette,
            "Themable",
            "All colors, radii, and tooltips are CSS variables. This page maps them to the demo's tokens.",
        ),
        (
            LucideGlyph::Accessibility,
            "Accessible",
            "ARIA grid, gridcell, dialog, and aria-selected wired up. Roving tabindex on cells.",
        ),
    ];

    view! {
        <div class="max-w-5xl w-full mx-auto px-4 mb-16">
            <h2 class="text-2xl font-semibold text-primary mb-1">
                "Built in"
            </h2>
            <p class="text-sm text-primary/60 mb-6">
                "Every feature is a prop you can opt out of."
            </p>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {features.iter().map(|(glyph, title, desc)| {
                    let g = *glyph;
                    view! {
                        <div class="flex flex-col gap-2 p-5 rounded-lg border border-border
                                    bg-card hover:border-highlight/40 transition-colors">
                            <div class="flex items-center justify-center w-10 h-10
                                        rounded-md bg-soft border border-border text-highlight mb-1">
                                <Icon glyph=g size="20" />
                            </div>
                            <h3 class="text-base font-semibold text-primary">{*title}</h3>
                            <p class="text-sm text-primary/60 leading-relaxed">{*desc}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
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
fn CodeSnippet() -> impl IntoView {
    view! {
        <div class="max-w-4xl w-full mx-auto px-4 mb-16">
            <h2 class="text-2xl font-semibold text-primary mb-1">
                "Eight lines from zero to a picker"
            </h2>
            <p class="text-sm text-primary/60 mb-5">
                "Single dependency. No CSS imports. Works in any Leptos 0.8 app."
            </p>
            <pre class="p-5 rounded-lg bg-secondary border border-border
                        overflow-x-auto text-xs leading-relaxed
                        font-mono text-primary/85">
                <code>{SNIPPET}</code>
            </pre>
            <div class="mt-3 text-xs text-primary/50 font-mono">
                "cargo add lepticons lepticons-picker"
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let count = LucideGlyph::count();
    view! {
        <div class="max-w-4xl w-full mx-auto px-4 pb-16 text-center">
            <a href="/" class="text-sm text-highlight hover:underline">
                {format!("← Browse all {} icons", count)}
            </a>
        </div>
    }
}
