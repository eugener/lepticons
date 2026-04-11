# lepticons

[Lucide](https://lucide.dev) icons for [Leptos](https://leptos.dev) applications.
Icons are grouped into 42 category features for selective compilation.

Part of the [Lepticons](https://github.com/eugener/lepticons) toolkit.

## Usage

```rust
use lepticons::{Icon, LucideGlyph};

// Basic usage
<Icon glyph=LucideGlyph::Search />

// With attributes
<Icon glyph=LucideGlyph::Heart class="text-red-500" size="32" stroke_width="2" />

// Reactive (icon changes based on state)
<Icon glyph=Signal::derive(move || {
    if dark_mode.get() { LucideGlyph::Moon } else { LucideGlyph::Sun }
}) />
```

## Installation

```toml
# All icons (default)
lepticons = "0.10"

# Only specific categories
lepticons = { version = "0.10", default-features = false, features = ["arrows", "navigation", "design"] }
```

Available categories: `accessibility`, `account`, `animals`, `arrows`, `buildings`, `charts`,
`communication`, `connectivity`, `cursors`, `design`, `development`, `devices`, `emoji`, `files`,
`finance`, `food_beverage`, `gaming`, `home`, `layout`, `mail`, `math`, `medical`, `multimedia`,
`nature`, `navigation`, `notifications`, `people`, `photography`, `science`, `seasons`, `security`,
`shapes`, `shopping`, `social`, `sports`, `sustainability`, `text`, `time`, `tools`,
`transportation`, `travel`, `weather`.

## Icon Component Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `glyph` | `impl Into<Signal<LucideGlyph>>` | required | The icon to render |
| `class` | `impl Into<TextProp>` | `""` | CSS class |
| `size` | `impl Into<TextProp>` | `"24"` | Width and height in pixels |
| `fill` | `impl Into<TextProp>` | `"none"` | SVG fill color |
| `stroke` | `impl Into<TextProp>` | `"currentColor"` | SVG stroke color |
| `stroke_width` | `impl Into<TextProp>` | `"1.5"` | SVG stroke width |

All string props accept `&str`, `String`, signals, or closures (`Fn() -> String`).
The `glyph` prop accepts a `LucideGlyph` value directly or a `Signal<LucideGlyph>`.

## Search and Categories

```rust
// Find icons by name, tag, or category
let results = LucideGlyph::find("arrow");

// Get all categories with icon counts
let categories = LucideGlyph::all_categories();

// Look up an icon by name
let icon = LucideGlyph::by_name("Activity");

// Total icon count
let total = LucideGlyph::count();

// Access icon metadata (zero-allocation, returns &'static str)
let svg = icon.svg();
let tags = icon.tags(); // returns iterator
let cats = icon.categories(); // returns iterator
```

## LucideGlyph

The `LucideGlyph` enum derives `Copy`, `Clone`, `Hash`, `Eq`, `Ord`, and `Debug`,
so it can be used as a HashMap key, in BTreeSets, and passed around without cloning.

## Rendering Modes

Lepticons works with all Leptos rendering modes (CSR, SSR, hydration).
No additional feature flags needed -- the library is rendering-mode agnostic.

## Related Crates

- [lepticons-picker](https://crates.io/crates/lepticons-picker) -- embeddable icon picker with search, grid, and category filter
- [lepticons-animate](https://crates.io/crates/lepticons-animate) -- stroke draw-in and CSS utility animations

## Leptos Compatibility

| Leptos | Lepticons |
|--------|-----------|
| 0.8.x  | 0.8.x -- 0.10.x |
| 0.6.x  | 0.5.x     |
| 0.5.x  | 0.4.x     |

## Demo

[lepticons.vercel.app](https://lepticons.vercel.app)

## License

MIT
