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
lepticons = "0.8"

# Only specific categories
lepticons = { version = "0.7", default-features = false, features = ["arrows", "navigation", "design"] }
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
| `class` | `&'static str` | `""` | CSS class |
| `size` | `&'static str` | `"24"` | Width and height in pixels |
| `fill` | `&'static str` | `"none"` | SVG fill color |
| `stroke` | `&'static str` | `"currentColor"` | SVG stroke color |
| `stroke_width` | `&'static str` | `"1.5"` | SVG stroke width |

The `glyph` prop accepts a `LucideGlyph` value directly (for static icons) or a
`Signal<LucideGlyph>` (for reactive icons that change based on state).

## Search and Categories

```rust
// Find icons by name, tag, or category
let results = LucideGlyph::find("arrow");

// Get all categories with icon counts
let categories = LucideGlyph::all_categories();

// Access icon metadata (zero-allocation, returns &'static str)
let svg = icon.svg();
let tags = icon.tags(); // returns iterator
let cats = icon.categories(); // returns iterator
```

## LucideGlyph

The `LucideGlyph` enum derives `Copy`, `Clone`, `Hash`, `Eq`, `Ord`, and `Debug`,
so it can be used as a HashMap key, in BTreeSets, and passed around without cloning.

## Leptos Compatibility

| Leptos | Lepticons |
|--------|-----------|
| 0.8.x  | 0.8.x     |
| 0.6.x  | 0.5.x     |
| 0.5.x  | 0.4.x     |

## Demo

[lepticons.vercel.app](https://lepticons.vercel.app)

## License

MIT
