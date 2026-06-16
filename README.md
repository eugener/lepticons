# Lepticons

[![Crates.io](https://img.shields.io/crates/v/lepticons.svg)](https://crates.io/crates/lepticons)
[![Downloads](https://img.shields.io/crates/d/lepticons.svg)](https://crates.io/crates/lepticons)
[![Docs.rs](https://docs.rs/lepticons/badge.svg)](https://docs.rs/lepticons)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rustc-1.81+-blue.svg)](#)
[![Leptos](https://img.shields.io/badge/leptos-0.8-blue.svg)](https://leptos.dev)

Icon toolkit for [Leptos](https://leptos.dev), powered by [Lucide](https://lucide.dev) icons. 1,700 icons, an embeddable picker, and animations -- shipped as `<Icon glyph=LucideGlyph::Search />`.

**[Live demo & icon browser at lepticons.9bits.cc](https://lepticons.9bits.cc)**

## Why lepticons?

- **Category features, not per-icon features.** 42 category flags cover ~1,700 icons. Pick `arrows + navigation + text` and the rest is dead code -- no `cargo build` time blowup from thousands of feature gates.
- **Always-fresh Lucide.** Weekly automated PR pulls the upstream `lucide` submodule and regenerates the icon data. No stale forks.
- **More than icons.** Drop-in `<IconPickerPopover>` for users who need to pick icons at runtime, plus stroke draw-in / spin / pulse animations.
- **Type-safe enum, no string lookups.** `LucideGlyph::Heart` is a `&'static str` under the hood; misspelling fails at compile time.
- **Framework-agnostic data layer.** Use `lepticons-data` standalone if you only need search + metadata without a Leptos rendering layer.

## Crates

| Crate | Description |
|-------|-------------|
| [`lepticons`](lepticons/) | Leptos `Icon` and `CustomIcon` components; re-exports the `lepticons-data` surface ([crates.io](https://crates.io/crates/lepticons), [docs.rs](https://docs.rs/lepticons)) |
| [`lepticons-data`](lepticons-data/) | Framework-agnostic data layer -- `LucideGlyph` enum, search, categories, 42 category features ([crates.io](https://crates.io/crates/lepticons-data), [docs.rs](https://docs.rs/lepticons-data)) |
| [`lepticons-picker`](lepticons-picker/) | Embeddable icon picker -- search, grid, category filter, popover ([crates.io](https://crates.io/crates/lepticons-picker), [docs.rs](https://docs.rs/lepticons-picker)) |
| [`lepticons-animate`](lepticons-animate/) | Icon animations -- stroke draw-in, spin, pulse, bounce, ping ([crates.io](https://crates.io/crates/lepticons-animate), [docs.rs](https://docs.rs/lepticons-animate)) |
| [`demo`](demo/) | Demo app deployed to [lepticons.9bits.cc](https://lepticons.9bits.cc) |
| [`codegen`](codegen/) | Code generator that reads Lucide SVGs and produces `lepticons-data/src/lucide_icon_data.rs` |

Most users only need `lepticons`; it re-exports everything from `lepticons-data`. Depend on `lepticons-data` directly only if you want the icon enum + search without a Leptos rendering layer.

## Quick start

```sh
cargo add lepticons
```

```rust
use lepticons::{Icon, LucideGlyph};

// Render an icon
view! { <Icon glyph=LucideGlyph::Search /> }

// With props
view! { <Icon glyph=LucideGlyph::Heart class="text-red-500" size="32" /> }
```

### Trim binary size with category features

The default feature set bundles all 42 categories. Opt out and pick only what you use:

```toml
[dependencies]
lepticons = { version = "0.12", default-features = false, features = [
    "arrows",
    "navigation",
    "text",
] }
```

See [`lepticons/Cargo.toml`](lepticons/Cargo.toml) for the full category list.

### Pick icons at runtime

```rust
use lepticons::LucideGlyph;
use lepticons_picker::IconPickerPopover;
use leptos::prelude::*;

let (icon, set_icon) = signal(None::<LucideGlyph>);

view! {
    <IconPickerPopover
        selected=icon
        on_select=Callback::new(move |g| set_icon.set(Some(g)))
    >
        <button>"Choose icon"</button>
    </IconPickerPopover>
}
```

See each crate's README and [docs.rs/lepticons](https://docs.rs/lepticons) for the full API.

## Building

```sh
# Library
cargo clippy -p lepticons --all-targets -- -D warnings && cargo test -p lepticons

# Demo (requires wasm32-unknown-unknown target)
cd demo && trunk serve

# Code generator (regenerates lepticons-data/src/lucide_icon_data.rs)
cd codegen && cargo run --bin lepticons-codegen
```

## Lucide submodule

```sh
git clone --recurse-submodules https://github.com/eugener/lepticons.git
# or after clone:
git submodule update --init
```

## License

MIT -- see [LICENSE](LICENSE).

Bundled Lucide icon assets are MIT-licensed by the Lucide / Feather authors. See [NOTICE](NOTICE) for attribution.
