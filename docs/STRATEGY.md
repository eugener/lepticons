# Lepticons Strategy: Icon Toolkit for Leptos

## Position

Lepticons is a Lucide icon library for Leptos with ~10 crates.io downloads. The dominant competitor is `leptos_icons` + `icondata` (222K+ downloads, 18+ icon sets). `lucide-leptos` (26K downloads) covers Lucide specifically for Leptos/Dioxus/Yew.

Raw icon rendering is commoditized -- all approaches produce the same SVG output.

## Differentiation

What lepticons has that others don't:

- Built-in zero-allocation search index (name, tags, categories)
- Rich metadata: tags, categories, contributors as typed iterators
- Category-based feature gating (semantically meaningful)
- Single enum (`LucideGlyph`) that is Copy/Hash/Ord, usable as map keys and match arms
- A polished demo app with search, browse, preview, and export

## Strategic Direction: Icon Toolkit

Stop competing on "render an SVG." Reposition as an icon toolkit for Leptos apps.

**Before:** "Lepticons is another way to render Lucide icons in Leptos."

**After:** "Lepticons is an icon toolkit for Leptos -- searchable picker, SSR support, animations, and the best icon reference site for Rust developers."

## Phases

See individual phase documents:

- [Phase 1: Core Library Improvements](phase-1-core.md) -- COMPLETE
- [Phase 2: IconPicker Crate](phase-2-picker.md) -- COMPLETE
- [Phase 3: Demo App Refactor](phase-3-demo.md) -- COMPLETE
- [Phase 4: Animations](phase-4-animations.md) -- COMPLETE

## Current Versions

| Crate | Version |
|-------|---------|
| `lepticons` | 0.10.0 |
| `lepticons-picker` | 0.2.0 |
| `lepticons-animate` | 0.1.1 |

## Discoverability

Ship quality, then promote:

1. PR to awesome-leptos (component libraries section)
2. Announce in Leptos Discord #showcase when picker ships
3. Blog post: "Building an Icon Picker in Leptos"
4. crates.io keywords: `icon-picker`, `lucide` on picker crate
5. README badges: crates.io version, docs.rs, demo link

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| `TextProp` changes break edge cases | `&str` auto-converts via `#[prop(into)]`; test all demo usages |
| `EnumString` + 700 variants = slow compile | Benchmark before/after; `EnumString` is a simple match |
| Picker CSS conflicts with user's Tailwind | All styles via CSS variables + `class` overrides; no global CSS |
| Popover positioning edge cases | CSS-only positioning in v0.1; add floating-ui in v0.2 if needed |
| Low adoption despite quality | Discoverability actions are the real lever |
