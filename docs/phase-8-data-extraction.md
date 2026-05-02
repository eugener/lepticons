# Phase 8: Data Extraction -- PLANNED

Target: new `lepticons-data` crate, no version bump for `lepticons` ABI but internal restructure.
Healthy architecture move that makes any future framework adapter feasible. ~2 evenings.
Independent of any decision to actually add a second framework.

## Goals

Decouple the icon **data** (enum, metadata, search, categories, raw SVG) from the **Leptos rendering layer**. Today they live together in `lepticons`; that conflates two concerns.

After Phase 8:

- `lepticons-data` -- pure data + search + categories. No Leptos dep. `no_std`-friendly where possible.
- `lepticons` -- Leptos `Icon` and `CustomIcon` components. Re-exports the public surface of `lepticons-data` for backwards compatibility.
- `lepticons-picker` -- depends on both, no behavior change.
- `lepticons-animate` -- depends on both, no behavior change.

## What Moves Where

**Stays in `lepticons` (Leptos-specific):**

- `Icon` component
- `CustomIcon` component (from Phase 5)
- `DEFAULT_SIZE`, `DEFAULT_FILL`, etc. (rendering defaults)
- Any `TextProp` / signal helpers

**Moves to `lepticons-data`:**

- `LucideGlyph` enum (with all strum derives)
- `lucide_icon_data.rs` (generated)
- `LucideGlyph::find`, `by_name`, `count`, `all_categories`, `svg`, `tags`, `categories`
- Category feature flags (the 42 of them)
- `OnceLock` search/category caches

## Re-Export Strategy

`lepticons` keeps its public API intact via re-exports:

```rust
// lepticons/src/lib.rs
pub use lepticons_data::{LucideGlyph, /* ... */};
```

Existing user code (`use lepticons::LucideGlyph;`) continues to work. No breaking change.

## Codegen Changes

`codegen/` regenerates into `lepticons-data/src/lucide_icon_data.rs` instead of `lepticons/src/lucide_icon_data.rs`. Update the workspace path.

The CI `update-icons.yml` workflow needs its path filter updated. Same with the `dependency chains` section in `CLAUDE.md`.

## Cargo.toml Sketch

```toml
# lepticons-data/Cargo.toml
[package]
name = "lepticons-data"
version = "0.1.0"
edition = "2021"
description = "Lucide icon data, search, and categories -- framework-agnostic core for the Lepticons toolkit."
license = "MIT"
repository = "https://github.com/eugener/lepticons"
homepage = "https://lepticons.9bits.cc"

[dependencies]
strum = { workspace = true }
strum_macros = { workspace = true }
# No leptos. No web-sys.

[features]
# Same 42 category flags as today; default = all.
```

```toml
# lepticons/Cargo.toml (changes)
[dependencies]
lepticons-data = { version = "0.1", path = "../lepticons-data" }
leptos = "0.8"
# Drop strum/strum_macros direct deps -- inherited via lepticons-data.

[features]
# Forward category flags to lepticons-data.
default = ["lepticons-data/default"]
arrows = ["lepticons-data/arrows"]
# ...etc for all 42
```

## Versioning

- `lepticons-data` -- new crate, ship as `0.1.0`
- `lepticons` -- bump to `0.11.0` (Phase 5 bump anyway; the re-export shuffle is non-breaking but a major-minor bump is the right signal that internals moved)
- `lepticons-picker` -- patch bump if its `lepticons` dep needs updating
- `lepticons-animate` -- same

## Migration Risk

- Public API: zero breakage if re-exports are exhaustive. Test by trying `cargo check` on the demo without modification.
- Compile time: should *improve* slightly because `lepticons-data` has no leptos macros.
- Bundle size: identical -- same code, same monomorphization.
- Doc generation: docs.rs builds two crates instead of one; symbols at `docs.rs/lepticons` still resolve via re-export.

## Why Now (or Not)

**Do this if:**
- Phase 9 (a real second framework adapter) becomes likely
- The codegen logic grows enough that mixing it with Leptos macros gets unwieldy
- Someone asks for "icon data without the rendering layer" (CLI tools, build scripts that lint icon usage, etc.)

**Skip this if:**
- Lepticons stays Leptos-only forever
- The split crate count (now 4 -> 5) is more confusing than helpful

The honest assessment: Phase 8 is **architectural hygiene without immediate payoff**. Worth doing before Phase 9; not worth doing in isolation.

## Acceptance

- `lepticons-data` crate published, `lepticons` re-exports preserve every existing public symbol
- Demo, picker, animate all build with no source changes
- `cargo bench` (if any) shows no regression on search index lookups
- CI `update-icons.yml` regenerates into the new crate path
