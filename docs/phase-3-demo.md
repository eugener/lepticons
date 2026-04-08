# Phase 3: Demo App Refactor

Goal: Refactor the demo app to consume `lepticons-picker`, add deep-linked icon pages, and improve the reference site quality.

## 3a. Consume lepticons-picker Components

### Current state

`demo/src/icons_view.rs` (480 lines) contains hand-rolled search, grid, category list, and detail panel.

### After refactor

Replace search, grid, and category list with picker components. Keep the detail panel (it's demo-specific with copy/export features).

`demo/src/icons_view.rs` shrinks from ~480 lines to ~150 lines:

```rust
use lepticons_picker::{IconSearch, IconGrid, CategoryFilter};

#[component]
pub fn IconsView() -> impl IntoView {
    let (filter, set_filter) = signal(String::new());
    let (debounced, set_debounced) = signal(String::new());
    let (selected, set_selected) = signal(None::<LucideGlyph>);

    // Close detail on filter change
    Effect::new(move |_| { filter.get(); set_selected.set(None); });

    // Close detail on Escape
    window_event_listener(keydown, move |ev: KeyboardEvent| {
        if ev.key() == "Escape" { set_selected.set(None); }
    });

    view! {
        <div class="flex flex-row">
            // Category sidebar
            <div class="w-64 flex-none bg-secondary h-screen overflow-y-auto">
                <StickyTop ...>
                    // Logo, icon count
                </StickyTop>
                <CategoryFilter
                    on_select=Callback::new(move |cat| set_filter.set(cat))
                    class="px-10 pt-5"
                />
            </div>

            // Main content
            <div class="px-10 mt-5 flex flex-col flex-auto h-screen overflow-y-auto">
                <StickyTop ...>
                    <MainMenu />
                    <IconSearch
                        value=filter
                        on_change=set_debounced
                        class="..."
                    />
                </StickyTop>
                <IconGrid
                    filter=debounced
                    selected=selected
                    on_select=Callback::new(move |icon| set_selected.set(Some(icon)))
                />
            </div>
        </div>

        // Detail panel stays in demo (copy/export features)
        <IconDetail selected_icon=selected set_selected_icon=set_selected />
    }
}
```

### What stays in the demo

- `IconDetail` component (copy SVG, copy data URL, download SVG, download PNG, copy JSX/Vue/Svelte/Angular/Leptos)
- `copy_to_clipboard()`, `download_blob()`, `download_png()` utility functions
- `StickyTop`, `ThemeToggle`, `MainMenu` components
- `LocalStorage` wrapper
- Layout, routing, license page

### demo/Cargo.toml change

```toml
[dependencies]
lepticons-picker = { path = "../lepticons-picker" }
```

## 3b. Deep-Linked Icon Pages

### New route

```rust
// main.rs
<Route path=path!("/icons/:name") view=IconPermalinkView />
```

### IconPermalinkView component

New file or section in `demo/src/icons_view.rs`:

```rust
#[component]
fn IconPermalinkView() -> impl IntoView {
    let params = use_params_map();
    let icon = move || {
        params.get().get("name")
            .and_then(|name| {
                // Try PascalCase first, then convert from kebab-case
                LucideGlyph::by_name(name)
                    .or_else(|| LucideGlyph::by_name(&name.to_case(Case::UpperCamel)))
            })
    };

    view! {
        {move || match icon() {
            Some(glyph) => view! {
                <div class="flex flex-col items-center justify-center min-h-screen p-10">
                    <Icon glyph=glyph size="128" stroke_width="1.5" />
                    <h1 class="text-3xl mt-8">{display_name(&glyph)}</h1>
                    // Tags, categories, usage snippet
                    <IconUsageSnippet glyph=glyph />
                </div>
            }.into_any(),
            None => view! { <NotFoundView /> }.into_any(),
        }}
    }
}
```

### URL format

- `/icons/Activity` -- PascalCase (matches enum variant)
- `/icons/arrow-right` -- kebab-case (converted to PascalCase for lookup)

Both resolve to the same icon.

### Link from grid

When clicking an icon in the grid, update the URL with `use_navigate()` so the browser URL reflects the selected icon. This enables:
- Copy-paste URL sharing
- Browser back/forward navigation
- Bookmarking specific icons

## 3c. Copy Leptos with Correct Feature Flag

### Current behavior

"Copy Leptos" button produces: `<Icon glyph=LucideGlyph::Activity />`

### Improved behavior

Show the full usage snippet including the required feature flag:

```rust
// Cargo.toml
// lepticons = { version = "0.9", default-features = false, features = ["medical"] }

use lepticons::{Icon, LucideGlyph};

view! { <Icon glyph=LucideGlyph::Activity /> }
```

### Implementation

Derive the minimal feature set from the icon's categories:

```rust
fn minimal_features(icon: &LucideGlyph) -> Vec<&str> {
    // Return the first category (alphabetically smallest feature name)
    // User needs any ONE of the icon's categories enabled
    icon.categories().take(1).collect()
}
```

Display in the detail panel as a copyable code block.

## 3d. Update Metadata Display

### LATEST_UPDATE.json

Generated by codegen, committed to repo:

```json
{
  "lucide_version": "0.500.0",
  "icon_count": 706,
  "updated": "2026-04-07"
}
```

Codegen update (`generate_lucide.rs`): write this file after generating icon data.

### Demo display

Show "706 icons -- Updated Apr 7, 2026" in the sidebar header. Currently shows just the count.

## 3e. File Changes Summary

| File | Change |
|------|--------|
| `demo/Cargo.toml` | Add `lepticons-picker` dependency |
| `demo/src/icons_view.rs` | Replace hand-rolled search/grid/categories with picker components (~330 lines removed) |
| `demo/src/main.rs` | Add `/icons/:name` route |
| `codegen/src/generate_lucide.rs` | Generate LATEST_UPDATE.json |

## 3f. Future Considerations (not in v0.1)

- **SSR for the demo app**: Move from Trunk (CSR) to cargo-leptos (SSR). Enables OG tags per icon page, faster first paint, SEO. Significant effort -- requires Axum setup, Vercel adapter for Rust SSR binary. Defer until picker ships and there's demand.
- **Mobile responsive layout**: Current layout assumes wide viewport. Add responsive breakpoints for the sidebar and grid.
- **Icon comparison view**: Select 2+ icons and view side-by-side at different sizes. Nice-to-have for designers.
