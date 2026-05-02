# Phase 7: Animations v2 -- PLANNED

Target: `lepticons-animate 0.2.0`. Hold until post-launch demand justifies the surface area. Largest creative scope of the planned phases; tackle one sub-feature per release.

## Goals

Extend the animation crate from "render once and play" (DrawIcon, CSS utilities) to "transition between states." The flagship feature is path morphing between two glyphs -- the `Menu` <-> `X` and `Sun` <-> `Moon` interactions every UI eventually wants.

## 7a. Path Morphing (flagship)

`<MorphIcon>` interpolates the SVG `d` attribute between two glyphs.

```rust
/// Animates between two glyphs by interpolating SVG path data.
///
/// Best results when the two glyphs have the same number of paths and
/// similar segment counts (e.g., `Menu` <-> `X`, `Sun` <-> `Moon`,
/// `Play` <-> `Pause`, `ChevronDown` <-> `ChevronUp`).
///
/// # Example
///
/// ```rust,ignore
/// let (open, set_open) = signal(false);
/// view! {
///     <button on:click=move |_| set_open.update(|o| *o = !*o)>
///         <MorphIcon
///             from=LucideGlyph::Menu
///             to=LucideGlyph::X
///             active=open
///             duration_ms=300
///         />
///     </button>
/// }
/// ```
#[component]
pub fn MorphIcon(
    #[prop(into)] from: Signal<LucideGlyph>,
    #[prop(into)] to: Signal<LucideGlyph>,
    /// When true, render `to`. When false, render `from`. Animates on change.
    #[prop(into)] active: Signal<bool>,
    #[prop(default = 300)] duration_ms: u32,
    #[prop(into, optional)] easing: Option<TextProp>,
    // ...standard Icon props
) -> impl IntoView
```

**Technique:**

1. Parse both glyphs' SVG content into normalized path lists
2. Match paths by index (path 0 of `from` <-> path 0 of `to`)
3. Use a path-interpolation lib (`lyon_path` or hand-rolled) to compute intermediate `d` strings at fixed sample points
4. CSS `transition: d` is now well-supported in Chrome/Safari; fall back to `requestAnimationFrame` updates on older browsers

**Constraints to document:**

- Glyphs with mismatched path counts produce a fade rather than a morph (graceful degradation)
- Glyphs with `<circle>` / `<line>` mixed with `<path>` need flattening to paths first
- Bundle cost: path-interpolation logic adds ~5-10KB to the animate crate; gate behind a `morph` feature flag if it grows

**Reference implementations:**

- [SMIL `animate` element](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animate) -- declarative, but deprecated path
- [Flubber](https://github.com/veltman/flubber) -- JS reference for SVG path interpolation
- [GreenSock MorphSVG](https://gsap.com/docs/v3/Plugins/MorphSVGPlugin/) -- proprietary, useful as design reference

## 7b. Color Tween

Animate stroke or fill color via CSS variables. No new component -- document the pattern in the animate crate's README.

```rust
view! {
    <Icon
        glyph=LucideGlyph::Heart
        class="transition-stroke duration-300"
        style="--stroke-color: var(--accent)"
        stroke="var(--stroke-color)"
    />
}
```

Add a small CSS utility set: `lepticons-color-tween-fast` (150ms), `-medium` (300ms), `-slow` (600ms).

Effort: small. Worth shipping with 7a.

## 7c. Stagger Helper

For animating groups of icons in sequence (nav menus, feature lists):

```rust
<StaggerGroup interval_ms=80>
    <DrawIcon glyph=LucideGlyph::Search />
    <DrawIcon glyph=LucideGlyph::Bell />
    <DrawIcon glyph=LucideGlyph::User />
    <DrawIcon glyph=LucideGlyph::Settings />
</StaggerGroup>
```

`StaggerGroup` injects `delay_ms` into each child's `DrawIcon` based on index. Implementation: clone children, add computed delay, render. Tricky in Leptos because props are consumed at render time -- may need a different API shape (e.g., `glyphs: Vec<LucideGlyph>` instead of children).

Defer this until 7a is shipped and we know the actual ergonomics.

## 7d. Badge Overlay

Common dashboard need: icon with a small numeric or dot badge.

```rust
<Icon glyph=LucideGlyph::Bell>
    <Badge count=3 />  // or <Badge dot />
</Icon>
```

Requires extending `Icon` to accept children -- which it doesn't today. Either:

- Add `children` prop to `Icon` (small breaking risk for users currently passing nothing)
- Ship as `<IconWithBadge glyph=... badge=BadgeKind::Count(3) />` separate component

Lean toward the separate component to avoid breaking `Icon`.

## Out of Scope (explicit non-goals)

### Lottie / dotLottie / Rive integration

**Decision: do not integrate.**

Lottie and friends are general animation runtimes for designer-authored animations (After Effects export pipelines). They solve a different problem than icon transitions.

Reasons against:

- **Aesthetic mismatch**: Lucide is 24px stroke icons. Lottie is filled illustrations with masks, gradients, motion paths. Mixing them in one app looks incoherent.
- **No production-quality Rust/WASM Lottie runtime**: `bodymovin-rs` is abandoned. Wrapping `lottie-web` adds 250KB JS dep + JS interop complexity. `dotlottie-rs` is viable but heavy.
- **Bundle cost**: even the lightest Lottie runtime is larger than the entire current lepticons WASM output.
- **Animation primitive is wrong-shaped**: Lottie shines for hero loaders and brand moments. Icons want sub-100ms GPU-cheap CSS-driven animations -- which is exactly what we already ship.
- **Identity dilution**: "Lucide icons + Lottie player for Leptos" is two products. Stay focused.

**If a user wants Lottie animations next to lepticons icons:** they should add `lottie-web` via a `<script>` tag in their app and place `<lottie-player>` elements wherever they want them. lepticons doesn't need to be in that loop.

**Narrow exception worth considering only if real demand emerges:** a tiny declarative JSON format describing icon-specific animation choreography (keyframes over a glyph's stroke paths). ~200 lines, no runtime dep, hand-authored. But path morphing covers 80% of the legitimate ask, so this stays parked.

### Multi-pack support (Tabler, Heroicons, Phosphor, etc.)

See STRATEGY.md. Solved by `leptos-icons + icondata`; we don't compete on icon-set count.

## Acceptance

Phase 7 ships incrementally. Each sub-feature can be its own minor release:

- 0.2.0: MorphIcon + ColorTween utilities
- 0.3.0: StaggerGroup (if API resolves cleanly)
- 0.4.0: IconWithBadge

Hold all of this until at least one of: (a) Phase 5 launch yields user requests for richer animation, (b) we've used these patterns ourselves in the demo for 2+ weeks and proven the API.
