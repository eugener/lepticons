# Lepticons

Icon toolkit for [Leptos](https://leptos.dev), powered by [Lucide](https://lucide.dev) icons.

Browse and search all icons at [lepticons.vercel.app](https://lepticons.vercel.app).

## Workspace

| Crate | Description |
|-------|-------------|
| [`lepticons`](lepticons/) | Core library -- icon component, search, metadata ([crates.io](https://crates.io/crates/lepticons), [docs.rs](https://docs.rs/lepticons)) |
| [`demo`](demo/) | Demo app deployed to Vercel |
| [`codegen`](codegen/) | Code generator that reads Lucide SVGs and produces `lucide_icon_data.rs` |

## Quick start

```rust
use lepticons::{Icon, LucideGlyph};

// Basic
<Icon glyph=LucideGlyph::Search />

// With props
<Icon glyph=LucideGlyph::Heart class="text-red-500" size="32" />
```

See [`lepticons/README.md`](lepticons/README.md) for full API docs and installation.

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
