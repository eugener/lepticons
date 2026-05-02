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
- [Phase 5: Launch Foundation](phase-5-foundation.md) -- PLANNED (target `lepticons 0.11.0`)
- [Phase 6: Picker Polish](phase-6-picker-polish.md) -- COMPLETE (`lepticons-picker 0.3.0`)
- [Phase 7: Animations v2](phase-7-animations-v2.md) -- PLANNED (target `lepticons-animate 0.2.0`)
- [Phase 8: Data Extraction](phase-8-data-extraction.md) -- PLANNED (architectural prep for multi-framework)
- [Phase 9: Framework Adapters](phase-9-framework-adapters.md) -- SPECULATIVE (gated on user signal)

## Current Versions

| Crate | Version |
|-------|---------|
| `lepticons` | 0.11.0 |
| `lepticons-picker` | 0.3.0 |
| `lepticons-animate` | 0.1.3 |

## Discoverability

Ship quality, then promote. See `phase-5-foundation.md` for the launch plan.

Done:
1. PR to awesome-leptos (component libraries section) -- listed
2. crates.io keywords: `icon-picker`, `lucide` on picker crate
3. README badges: crates.io version, docs.rs, demo link
4. Custom canonical domain: `lepticons.9bits.cc` (vercel.app 308s to it)

Pending (Phase 5 launch):
1. Leptos Discord `#showcase` post (day 0)
2. r/rust showcase post (day 0, Tue/Wed ~10am ET, lead with DrawIcon gif)
3. dev.to follow-up technique post (week 2-3, evergreen SEO)

Skip:
- Hacker News -- too unforgiving for library showcase posts
- dev.to as primary launch venue -- audience misalignment (mostly JS/TS devs)

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| `TextProp` changes break edge cases | `&str` auto-converts via `#[prop(into)]`; test all demo usages |
| `EnumString` + 700 variants = slow compile | Benchmark before/after; `EnumString` is a simple match |
| Picker CSS conflicts with user's Tailwind | All styles via CSS variables + `class` overrides; no global CSS |
| Popover positioning edge cases | CSS-only positioning in v0.1; add floating-ui in v0.2 if needed |
| Low adoption despite quality | Discoverability actions are the real lever |
