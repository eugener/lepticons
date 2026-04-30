# Lepticons

Icon toolkit for [Leptos](https://leptos.dev), powered by [Lucide](https://lucide.dev) icons.

Browse and search all icons at [lepticons.9bits.cc](https://lepticons.9bits.cc).

## Crates

| Crate | Description |
|-------|-------------|
| [`lepticons`](lepticons/) | Core library -- Icon component, search, metadata, category features ([crates.io](https://crates.io/crates/lepticons), [docs.rs](https://docs.rs/lepticons)) |
| [`lepticons-picker`](lepticons-picker/) | Embeddable icon picker -- search, grid, category filter, popover ([crates.io](https://crates.io/crates/lepticons-picker), [docs.rs](https://docs.rs/lepticons-picker)) |
| [`lepticons-animate`](lepticons-animate/) | Icon animations -- stroke draw-in, spin, pulse, bounce, ping ([crates.io](https://crates.io/crates/lepticons-animate), [docs.rs](https://docs.rs/lepticons-animate)) |
| [`demo`](demo/) | Demo app deployed to [lepticons.9bits.cc](https://lepticons.9bits.cc) |
| [`codegen`](codegen/) | Code generator that reads Lucide SVGs and produces `lucide_icon_data.rs` |

## Quick start

```rust
use lepticons::{Icon, LucideGlyph};

// Render an icon
<Icon glyph=LucideGlyph::Search />

// With props
<Icon glyph=LucideGlyph::Heart class="text-red-500" size="32" />
```

See each crate's README for full API docs.

## Building

```sh
# Library
cargo clippy -p lepticons --all-targets -- -D warnings && cargo test -p lepticons

# Demo (requires wasm32-unknown-unknown target)
cd demo && trunk serve

# Code generator (regenerates lepticons/src/lucide_icon_data.rs)
cd codegen && cargo run --bin lepticons-codegen
```

## Lucide submodule

```sh
git clone --recurse-submodules https://github.com/eugener/lepticons.git
# or after clone:
git submodule update --init
```

## License

MIT
