use leptos::prelude::*;
use leptos_meta::*;

use lepticons::*;

use crate::local_storage::LocalStorage;

/// Shared help-overlay signal, provided via context. Pages don't own this --
/// the App component creates and provides it once, so MainMenu's `?` button
/// and the App-level KeyboardHelp overlay stay in sync across routes.
#[derive(Clone, Copy)]
pub struct HelpOpen {
    pub read: ReadSignal<bool>,
    pub write: WriteSignal<bool>,
}

impl HelpOpen {
    /// Creates a `(false)` signal pair and provides it via context.
    pub fn provide() -> Self {
        let (read, write) = signal(false);
        let help = Self { read, write };
        provide_context(help);
        help
    }
}

/// Keyboard-shortcut overlay. Lists the global shortcuts (`/` to focus
/// search, `?` to toggle this dialog, arrow keys / Home / End / PageUp /
/// PageDown / Enter / Esc inside the icon grid). Same content on every
/// page so users get a consistent reference.
#[component]
pub fn KeyboardHelp(on_close: Callback<()>) -> impl IntoView {
    let rows: [(&str, &str); 8] = [
        ("/", "Focus search"),
        ("?", "Toggle this help"),
        ("Arrow keys", "Move focus across the grid"),
        ("Home / End", "Jump to first / last icon"),
        ("Page Up / Down", "Move five rows at a time"),
        ("Enter", "Open the focused icon's detail"),
        ("Esc", "Close the detail drawer or this help"),
        ("Click", "Select an icon (or open it from Recent)"),
    ];
    view! {
        <div
            class="fixed inset-0 z-[70] bg-primary/40 flex items-center justify-center p-6"
            role="dialog"
            aria-modal="true"
            aria-label="Keyboard shortcuts"
            on:click=move |_| on_close.run(())
        >
            <div
                class="bg-background border border-primary/15 rounded-lg shadow-xl
                       w-full max-w-md p-6"
                on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()
            >
                <div class="flex flex-row items-start justify-between mb-4">
                    <div>
                        <div class="text-[0.6875rem] uppercase tracking-wider text-highlight font-medium">
                            "Keyboard"
                        </div>
                        <h2 class="text-lg font-semibold text-primary mt-0.5">
                            "Shortcuts"
                        </h2>
                    </div>
                    <button
                        class="flex-none p-1 text-primary/50 hover:text-primary"
                        aria-label="Close shortcuts"
                        on:click=move |_| on_close.run(())
                    >
                        <Icon glyph=LucideGlyph::X size="18"/>
                    </button>
                </div>
                <div class="flex flex-col gap-2">
                    {rows.iter().map(|(keys, label)| view! {
                        <div class="flex flex-row items-center justify-between gap-3 py-1
                                    border-b border-primary/5 last:border-0">
                            <span class="text-sm text-primary/70">{*label}</span>
                            <kbd class="px-2 py-1 text-[0.6875rem] font-mono text-primary/70
                                        bg-primary/5 border border-primary/15 rounded">
                                {*keys}
                            </kbd>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
                <p class="mt-4 text-xs text-primary/45">
                    "Tip: shortcuts are inactive while you're typing in an input."
                </p>
            </div>
        </div>
    }
}

// StickyTop is a component that sticks to the top of the screen.
#[component]
pub fn StickyTop(#[prop(default = "")] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class={format!("sticky top-0 z-50 {}", class)}>
            {children()}
        </div>
    }
}

const DARK_MODE: &str = "dark-mode";

/// Shared dark mode signal, provided via context.
#[derive(Clone, Copy)]
pub struct DarkMode {
    pub read: ReadSignal<bool>,
    pub write: WriteSignal<bool>,
}

impl DarkMode {
    /// Creates and provides the dark mode signal via context.
    pub fn provide() -> Self {
        let (read, write) = signal(LocalStorage::get(DARK_MODE).unwrap_or(false));
        let dm = Self { read, write };
        provide_context(dm);
        dm
    }
}

// ThemeToggle is a component that toggles between light and dark mode.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let dm = use_context::<DarkMode>().expect("DarkMode context");
    let dark_mode = dm.read;
    let set_dark_mode = dm.write;

    Effect::new(move |_| {
        LocalStorage::set(DARK_MODE, &dark_mode.get());
    });

    let theme = move || if dark_mode.get() { "dark" } else { "light" }.to_string();

    let pos_class = move || {
        format!(
            "flex flex-row gap-2 items-center {} w-12 h-6 bg-primary/50 rounded-full",
            if dark_mode.get() {
                "justify-start"
            } else {
                "justify-end"
            }
        )
    };

    let toggle_theme = move |_| set_dark_mode.set(!dark_mode.get());

    let theme_glyph = move || {
        if dark_mode.get() {
            LucideGlyph::Moon
        } else {
            LucideGlyph::Sun
        }
    };

    view! {
        <Html {..} class=theme />
        <button class=pos_class
             on:click=toggle_theme >
            <div class="flex-none w-6 h-6 bg-primary/100 rounded-full">
                <Icon glyph=Signal::derive(theme_glyph) class="text-secondary p-1"/>
            </div>
        </button>
    }
}
