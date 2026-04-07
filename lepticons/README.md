# Lepticons

Add 1694 [Lucide](https://lucide.dev) icons to your [Leptos](https://leptos.dev) projects.
Icons are grouped into 42 category features, so you can include only what you need.

## Usage

```rust
use lepticons::*;

// Basic usage
<Icon<LucideGlyph> glyph=Signal::derive(move || LucideGlyph::Search) />

// With attributes
<Icon<LucideGlyph>
    glyph=Signal::derive(move || LucideGlyph::Heart)
    class="text-red-500"
    size="32"
    stroke_width="2"
/>
```

## Installation

Add to your `Cargo.toml`:

```toml
# All icons (default)
lepticons = "0.6"

# Only specific categories
lepticons = { version = "0.6", default-features = false, features = ["arrows", "navigation", "design"] }
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
| `glyph` | `Signal<T>` | required | The icon to render |
| `class` | `&'static str` | `""` | CSS class |
| `size` | `&'static str` | `"24"` | Width and height in pixels |
| `fill` | `&'static str` | `"none"` | SVG fill color |
| `stroke` | `&'static str` | `"currentColor"` | SVG stroke color |
| `stroke_width` | `&'static str` | `"1.5"` | SVG stroke width |

## Search and Categories

```rust
// Find icons by name, tag, or category
let results = LucideGlyph::find("arrow");

// Get all categories with icon counts
let categories = LucideGlyph::all_categories();
```

## Leptos Compatibility

| Leptos | Lepticons |
|--------|-----------|
| 0.8.x  | 0.6.x     |
| 0.6.x  | 0.5.x     |
| 0.5.x  | 0.4.x     |

## Demo

[lepticons.vercel.app](https://lepticons.vercel.app)

## License

MIT
