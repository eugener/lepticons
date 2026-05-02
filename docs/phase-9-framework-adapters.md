# Phase 9: Framework Adapters -- SPECULATIVE

Target: new `lepticons-dioxus` crate as a first proof point.
**Do not start until Phase 8 ships and at least one concrete user request lands** ("can I use lepticons in Dioxus?", "any plans for GPUI?", etc.).

Most projects that pivot to "multi-framework" before they have signal die from maintenance burden. Wait for signal.

## Trigger Conditions

Start Phase 9 only when **two or more** of the following hold:

- A user opens an issue asking for support in framework X (Dioxus, Slint, Iced, egui, GPUI, Yew)
- Phase 5 launch traffic shows search-engine queries like "lucide icons dioxus rust" landing on lepticons
- A maintainer of another Rust UI framework reaches out to integrate
- We have free maintenance bandwidth to support a second target indefinitely

If only one of these triggers, write a tracking issue and wait.

## Adapter Priority

Ranked by audience size, technical fit, and effort:

| Order | Framework | Reason |
|-------|-----------|--------|
| 1 | **Dioxus** | Largest audience after Leptos. Same SVG model. Probably ~1 day of work. |
| 2 | **Yew** | Mature, web SVG. Smaller audience than Dioxus. ~1-2 days. |
| 3 | **Slint** | Real desktop/embedded niche. SVG via `<Image>`. ~3-5 days, different image model. |
| 4 | **Iced** | Rust desktop standard but icon rendering is non-trivial -- needs `iced_graphics` path API. ~1 week. |
| 5 | **egui** | Huge user base for tools/plotting. SVG via `egui_extras` + `usvg`. Mid effort. |
| 6 | **GPUI** | Tiny audience today, no stable public API on crates.io, custom GPU renderer requires from-scratch path conversion. Last on the list until Zed extensions create real demand. |

Stop after the first adapter unless additional signal arrives.

## Adapter Crate Pattern

Each adapter follows the same shape:

```toml
# lepticons-{framework}/Cargo.toml
[package]
name = "lepticons-dioxus"  # or -yew, -slint, etc.
version = "0.1.0"
description = "Lucide icons for {Framework} apps, powered by lepticons-data."

[dependencies]
lepticons-data = { version = "0.1", path = "../lepticons-data" }
{framework} = "..."
```

The adapter exposes an `Icon` (and `CustomIcon`) component idiomatic to that framework. Nothing else from `lepticons` (Leptos-specific) is reused.

```rust
// Conceptual: lepticons-dioxus
use lepticons_data::LucideGlyph;
use dioxus::prelude::*;

#[component]
pub fn Icon(glyph: LucideGlyph, /* ... */) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            // ...same defaults as lepticons
            dangerous_inner_html: "{glyph.svg()}"
        }
    }
}
```

## What Does NOT Get Ported

- **The picker** stays Leptos-only. Re-implementing `IconPicker` in Dioxus/Yew/Slint is a large component port -- different reactivity, different DOM access patterns. If users want a picker in their non-Leptos framework, they can write one against `lepticons-data` themselves.
- **The animation crate** stays Leptos-only for the same reason. `DrawIcon` uses `web-sys` SVG DOM access wired to Leptos lifecycles.
- **The demo app** stays Leptos. Don't fork it.

This is intentional. The thing that ports cleanly is the *icon data + simple `<Icon>` component*. The ecosystem (picker, animations) is the Leptos differentiator and stays there.

## GPUI: Special Case

If GPUI demand materializes (Zed extensions need icons), evaluate carefully:

1. **Wait for GPUI to publish a stable crate to crates.io.** Building against the Zed monorepo path is not viable for a published crate.
2. **Choose a rendering strategy:**
   - Convert SVG paths to GPUI's path API at build time (codegen extension) -- best fidelity, lots of code
   - Embed `resvg` to rasterize at runtime -- simple but heavy
   - Ship a font with icons as glyphs (like Lucide does for `lucide-static`) -- might be the easiest path if GPUI has good text rendering
3. **Don't try to "share components" with Leptos.** GPUI's immediate-mode entity model is fundamentally different. Treat it as a separate adapter.
4. **Time-box.** If a working `<Icon>` in GPUI takes more than 5 days, abandon and document why.

## Multi-Pack Reminder

Phase 9 is about supporting other **frameworks**, not other **icon packs**. We still ship Lucide only. If someone asks for Tabler/Heroicons/Phosphor support, redirect them to `leptos-icons + icondata` (web) or `iconify` (multi-platform JS). Stay focused on the Lucide-for-Rust niche.

## Versioning Strategy

Each adapter crate has its own version, independent of `lepticons`. They all depend on `lepticons-data` for the icon set.

When `lepticons-data` ships a new version (new icons, new metadata), every adapter that wants those icons bumps its `lepticons-data` dep and ships a patch release. Coordinate via a release script in `codegen/`.

## Acceptance (per adapter)

- `<Icon>` component renders any `LucideGlyph` with the same defaults as `lepticons` (24px, stroke currentColor, stroke-width 1.5)
- `<CustomIcon>` accepts inline SVG content
- Published to crates.io with `repository`, `homepage = "https://lepticons.9bits.cc"`, license MIT
- README documents framework version compatibility (e.g., "Dioxus 0.6.x")
- Listed in the relevant awesome-{framework} repo
- One small example in `examples/{framework}/`

## Acceptance (overall Phase 9)

Ship the first adapter (almost certainly Dioxus). Watch downloads + GitHub activity for 60 days. If <100 downloads or no issues filed, the multi-framework experiment failed -- archive the adapter crate, document the lesson, return focus to Leptos.

If >500 downloads or active issues, queue the second adapter from the priority list.
