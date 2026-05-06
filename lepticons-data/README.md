# lepticons-data

Framework-agnostic Lucide icon data, search, and categories for the [Lepticons](https://github.com/eugener/lepticons) toolkit.

This crate holds the `LucideGlyph` enum (one variant per Lucide icon, ~1700 icons), the 42 category features for selective compilation, a cached substring search index, and a category aggregation cache. It has no Leptos or `web-sys` dependency.

Most users should depend on [`lepticons`](https://crates.io/crates/lepticons) instead, which re-exports everything here and adds the Leptos `Icon` and `CustomIcon` components. Reach for `lepticons-data` directly only if you want icon data without a rendering layer -- CLI tools, build scripts, custom lints, or an alternative framework adapter.

## Usage

```rust
use lepticons_data::{Glyph, LucideGlyph};
use lepticons_data::strum::IntoEnumIterator;

// Look up an icon by name (returns Option<LucideGlyph>)
let icon = LucideGlyph::by_name("Activity").unwrap();

// Total icon count (cached)
let total = LucideGlyph::count();

// Iterate all icons enabled by your category features
for glyph in LucideGlyph::iter() {
    let _ = glyph.svg();        // &'static str -- inner SVG
    let _ = glyph.tags();       // iterator of &'static str
    let _ = glyph.categories(); // iterator of &'static str
}

// Search by name + tags + categories (case-insensitive, AND across words)
let results = LucideGlyph::find("arrow up");

// All categories with icon counts (cached)
for (category, count) in LucideGlyph::all_categories() {
    println!("{category}: {count}");
}
```

The `Glyph` trait abstracts the SVG-content lookup so renderers can accept any glyph type, not only `LucideGlyph`.

## Installation

```toml
# All icons (default)
lepticons-data = "0.1"

# Only specific categories
lepticons-data = { version = "0.1", default-features = false, features = ["arrows", "navigation"] }
```

Available categories: `accessibility`, `account`, `animals`, `arrows`, `buildings`, `charts`, `communication`, `connectivity`, `cursors`, `design`, `development`, `devices`, `emoji`, `files`, `finance`, `food_beverage`, `gaming`, `home`, `layout`, `mail`, `math`, `medical`, `multimedia`, `nature`, `navigation`, `notifications`, `people`, `photography`, `science`, `seasons`, `security`, `shapes`, `shopping`, `social`, `sports`, `sustainability`, `text`, `time`, `tools`, `transportation`, `travel`, `weather`.

## License

MIT
