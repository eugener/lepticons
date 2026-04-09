# Lepticons

Add [Lucide](https://lucide.dev) icons to your [Leptos](https://leptos.dev) projects.
Icons are grouped into 42 category features, so you can include only what you need.

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

Add to your `Cargo.toml`:

```toml
# All icons (default)
lepticons = "0.9"

# Only specific categories
lepticons = { version = "0.9", default-features = false, features = ["arrows", "navigation", "design"] }
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

## Leptos Compatibility

| Leptos | Lepticons |
|--------|-----------|
| 0.8.x  | 0.8.x, 0.9.x |
| 0.6.x  | 0.5.x     |
| 0.5.x  | 0.4.x     |

## Demo

[lepticons.vercel.app](https://lepticons.vercel.app)

## License

MIT
