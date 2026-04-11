# lepticons-animate

Icon animations for [Leptos](https://leptos.dev) -- stroke draw-in, spin, pulse, bounce.

Part of the [Lepticons](https://github.com/eugener/lepticons) toolkit.

## DrawIcon

Stroke draw-in animation. The icon's paths animate from invisible to fully drawn.

```rust
use lepticons::LucideGlyph;
use lepticons_animate::DrawIcon;

<DrawIcon glyph=LucideGlyph::Check duration_ms=500 />

// With delay and custom easing
<DrawIcon glyph=LucideGlyph::Heart duration_ms=800 delay_ms=200 easing="ease-out" />
```

Props: `glyph`, `duration_ms` (default 600), `delay_ms` (default 0), `easing` (default "ease-in-out"), `class`, `size`, `fill`, `stroke`, `stroke_width`.

## CSS Utility Animations

Add `<AnimationStyles />` once in your app, then use class names on any `<Icon>`:

```rust
use lepticons::{Icon, LucideGlyph};
use lepticons_animate::AnimationStyles;

view! {
    <AnimationStyles />
    <Icon glyph=LucideGlyph::Loader class="lepticons-spin" />
    <Icon glyph=LucideGlyph::Bell class="lepticons-bounce" />
    <Icon glyph=LucideGlyph::Heart class="lepticons-pulse" />
    <Icon glyph=LucideGlyph::Radio class="lepticons-ping" />
}
```

| Class | Effect |
|-------|--------|
| `lepticons-spin` | 360 degree rotation, 1s linear infinite |
| `lepticons-pulse` | Opacity fade 1 to 0.5, 2s ease-in-out infinite |
| `lepticons-bounce` | Vertical bounce, 1s ease infinite |
| `lepticons-ping` | Scale up and fade out, 1s infinite |

## Related Crates

- [lepticons](https://crates.io/crates/lepticons) -- core icon library (required dependency)
- [lepticons-picker](https://crates.io/crates/lepticons-picker) -- embeddable icon picker with search, grid, and category filter

## Requirements

- Leptos 0.8.x
- lepticons 0.9.x

## Demo

[lepticons.vercel.app](https://lepticons.vercel.app)

## License

MIT
