# Phase 5: Launch Foundation -- PLANNED

Target: `lepticons 0.11.0`. Pre-launch hardening so the upcoming r/rust post has a credible story. ~1-2 evenings of work.

## Goals

1. Ship a custom-icon escape hatch so users aren't blocked when Lucide is missing a glyph
2. Document SSR/CSR/hydrate explicitly with worked examples
3. Audit the demo for places we still ship inline SVG instead of dogfooding
4. Launch publicly on r/rust with a complementary dev.to follow-up 2-3 weeks later

## 5a. CustomIcon Component

`<CustomIcon>` lives in the `lepticons` crate alongside `Icon`. It accepts arbitrary SVG inner content and renders with the same prop API. This keeps `LucideGlyph` clean (Copy/Hash/Ord/Eq stay valid; no string payloads in match arms).

```rust
/// Renders an arbitrary inline SVG using the same prop API as `Icon`.
///
/// Use for in-house icons, brand logos, or icons missing from Lucide.
/// The `svg` prop is the inner SVG markup (paths, circles, etc.) without
/// the wrapping `<svg>` element.
///
/// # Example
///
/// ```rust,ignore
/// const COMPANY_LOGO: &str = r#"<path d="M12 2L2 7l10 5 10-5-10-5z" />"#;
/// <CustomIcon svg=COMPANY_LOGO class="text-primary" size="32" />
/// ```
#[component]
pub fn CustomIcon(
    #[prop(into)] svg: TextProp,
    #[prop(into, optional)] class: Option<TextProp>,
    #[prop(into, optional)] size: Option<TextProp>,
    #[prop(into, optional)] fill: Option<TextProp>,
    #[prop(into, optional)] stroke: Option<TextProp>,
    #[prop(into, optional)] stroke_width: Option<TextProp>,
) -> impl IntoView
```

**Implementation notes:**

- Reuses the same wrapper SVG element + viewBox (`0 0 24 24`) as `Icon`
- `svg` content set via `inner_html` (matches existing `Icon` rendering)
- Defaults match `Icon`: size 24, fill none, stroke currentColor, stroke-width 1.5
- Document the 24x24 viewBox assumption -- callers must scale their SVG to match

**Decision:** Ship as `<CustomIcon>` not `LucideGlyph::Custom(...)`. Adding a `Custom(&'static str)` variant would force every existing `match LucideGlyph` to add an arm and break `Copy` (or force `'static` payloads, which leaks lifetime concerns).

## 5b. Rendering Modes Documentation

Add a top-level `docs/rendering-modes.md` plus a section in `lepticons/README.md`. Today the README claims "rendering-mode agnostic" without proof.

Cover:

- **CSR (default)**: Trunk + `cargo build --target wasm32-unknown-unknown` -- already in main README
- **SSR + hydration**: Axum server-fn template, ensure `inner_html` round-trips through hydration without warnings
- **Static SSR**: leptos `0.8` static-site export
- **DrawIcon caveat**: requires DOM access, flag that it only renders post-hydration in SSR mode
- **Picker caveat**: signals + click handlers require hydration

Validate by building a minimal Axum SSR app under `examples/ssr-axum/` (new directory). Don't add to the workspace -- keep it as a standalone example users can copy.

## 5c. Demo Dogfooding Audit

Walk through `demo/src/` and replace any inline SVG that has a Lucide equivalent with `<Icon>`. Known cases:

- `menu.rs`: GitHub link -- **keep inline** (Lucide dropped brand logos, comment already explains)
- Any spinners, search/clear glyphs, theme-toggle icons -- replace with `<Icon>` calls

This is housekeeping; small diff, good demo aesthetic.

## 5d. Launch Plan

After 0.11.0 is on crates.io with the changes above:

**Day 0 (Tuesday or Wednesday, ~10am ET)**
- Drop in Leptos Discord `#showcase` -- get 5-10 friendly first reactions before the wider audience sees the post
- Post to `r/rust` with title pattern: `"Lepticons 0.11: Lucide icons for Leptos with stroke draw-in animations and a built-in picker"`
- Lead with a 5-second gif of `DrawIcon` cycling 4-5 glyphs -- this is the differentiator nothing else has
- Body: 3-4 sentence pitch + bullet list of features + links (crates.io, demo, GitHub)
- Engage in comments for the first 3-4 hours; answer technical questions, accept criticism, file resulting GitHub issues

**Skip:** Hacker News (too unforgiving for library showcase posts), cross-posting to dev.to same week

**Day 14-21:** Write a dev.to follow-up. Different shape -- a *technique* post, not an announcement. Candidates:
- `"Building SVG stroke draw-in animations in Rust/WASM"`
- `"Generating 700 typed icon enums from a Lucide submodule with strum"`

dev.to is evergreen and Google-indexed; framing matters more than timing.

## Out of Scope for Phase 5

- Multi-pack support (Tabler, Heroicons, etc.) -- diluting the "Lucide for Leptos" identity, see STRATEGY.md
- Lottie/Rive runtime integration -- wrong tool for icons (~250KB runtime, aesthetic mismatch with stroke icons)
- Theming system beyond existing CSS-variable setup
- Picker UX changes -- deferred to Phase 6

## Acceptance

- `<CustomIcon>` shipped, documented, has a doctest
- `docs/rendering-modes.md` exists with worked SSR example
- All Lucide-replaceable inline SVGs in demo replaced
- 0.11.0 published on crates.io
- r/rust post published, primary feedback triaged into GitHub issues
