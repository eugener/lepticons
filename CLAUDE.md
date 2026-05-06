# Lepticons

Icon toolkit for Leptos. 6 crates: `lepticons-data/` (framework-agnostic data + search + categories), `lepticons/` (Leptos renderer + re-exports), `lepticons-picker/`, `lepticons-animate/`, `demo/`, `codegen/`.

## Feedback Loop

After ANY code change, run before considering done:

```sh
# Data layer (no Leptos)
cargo clippy -p lepticons-data --all-targets -- -D warnings && cargo test -p lepticons-data

# Library
cargo clippy -p lepticons --all-targets -- -D warnings && cargo test -p lepticons

# Picker
cargo clippy -p lepticons-picker --all-targets -- -D warnings

# Animate
cargo clippy -p lepticons-animate --all-targets -- -D warnings && cargo test -p lepticons-animate

# Demo
cargo clippy -p lucide-icons-demo --all-targets -- -D warnings

# Full workspace (CI-equivalent)
cargo clippy --all-targets -- -D warnings && cargo test --workspace

# WASM (when touching component rendering)
cargo build --target wasm32-unknown-unknown -p lucide-icons-demo

# Codegen (after modifying generator)
cd codegen && cargo run --bin lepticons-codegen && cd .. && cargo clippy -p lepticons-data -p lepticons --all-targets -- -D warnings && cargo test -p lepticons-data -p lepticons

# Update Lucide icons (manual)
cd lucide && git fetch origin && git checkout origin/main && cd .. && cd codegen && cargo run --bin lepticons-codegen && cd .. && cargo clippy -p lepticons-data -p lepticons --all-targets -- -D warnings && cargo test -p lepticons-data -p lepticons

# Demo dev server
cd demo && trunk serve
```

## Dependency Chains

- `codegen/` changed -> re-run generator -> verify lepticons-data and lepticons compile
- `lucide/` submodule updated -> re-run generator (rewrites both `lepticons-data/Cargo.toml` and `lepticons/Cargo.toml` feature tables)
- `lepticons-data/` API changed -> re-export through `lepticons/src/lib.rs` and verify picker/animate compile
- `lepticons/` API changed -> verify picker and animate still compile
- `lepticons/src/lib.rs` props changed -> update `lepticons/README.md` props table
- `lepticons-data/Cargo.toml` features changed -> codegen also rewrites the matching forwarders in `lepticons/Cargo.toml`; update `lepticons/README.md` categories list

## Design Decisions (do not regress)

- Strum derives (`EnumIter`, `EnumProperty`, `EnumString`, `IntoStaticStr`) for icon metadata
- Category features, not per-icon features (42 vs 1694)
- `OnceLock` for search index and categories cache
- `inner_html` for SVG content in Icon component
- `&'static str` from `IntoStaticStr` for `name()`, strum props for `svg()`/`tags_str()`/`categories_str()`
- Library is rendering-mode agnostic -- no `csr`/`ssr` features
- Picker uses inline styles + `--lp-*` CSS custom properties, no CSS framework
- DrawIcon wraps SVG in div (Leptos 0.8 has no SVG NodeRef)
- Easing is an enum, not a string
- `Callback<T>` for event props, not `WriteSignal<T>`
- Highlight color via `--highlight` CSS variable in demo

## Conventions

- Leptos 0.8.x -- `leptos::prelude::*`
- Props: `#[prop(into)]` for signals, `#[prop(into, optional)]` for optional
- Event props: `Callback<T>`, not `WriteSignal<T>`
- Document all `pub` items with `///`
- Prefer `&'static str` over `String` in library APIs
- Export shared constants (`DEFAULT_SIZE`, etc.) from `lepticons`
- Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`, `ci:`, `test:`, `perf:`, `style:`

## Pitfalls

- `lepticons-data/src/lucide_icon_data.rs` is generated -- never edit, run `codegen` to regenerate
- `lepticons-data` is `publish = false`; the public surface ships through `lepticons` re-exports
- `lucide/` is a git submodule -- `git clone --recurse-submodules`
- `use leptos::ev::*` causes name conflicts (`input`, `reset`, `toggle`) -- use selective imports
- `TextProp` is Clone not Copy -- clone for dual use, `StoredValue` in reactive closures

## Git Workflow

- `develop`: working branch, PRs target here
- `master`: production, push triggers Vercel deploy
- Tags (`v*`): trigger crates.io publish in order -- `lepticons-data`, `lepticons`, `lepticons-picker`, `lepticons-animate`
- Weekly automated PR updates `lucide/` submodule

## Publishing

1. Bump versions in all changed crate Cargo.toml files
2. Update dependency versions in picker/animate if lepticons version changed
3. Update README version references
4. Merge develop to master
5. `git tag v0.X.0 && git push origin v0.X.0`
