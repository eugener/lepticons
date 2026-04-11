# Lepticons

Icon toolkit for Leptos (not just icon rendering). See `docs/STRATEGY.md` for positioning and `docs/phase-*.md` for implementation plans.

## Feedback Loop

After ANY code change, run the appropriate verification before considering the task done.

### Library changes (`lepticons/`)

```sh
cargo clippy -p lepticons --all-targets -- -D warnings && cargo test -p lepticons
```

### Demo app changes (`demo/`)

```sh
cargo clippy -p lucide-icons-demo --all-targets -- -D warnings && cargo test -p lucide-icons-demo
```

### Full workspace (CI-equivalent)

```sh
cargo clippy --all-targets -- -D warnings && cargo test --workspace
```

### WASM compilation (run when touching any component rendering code)

```sh
cargo build --target wasm32-unknown-unknown -p lucide-icons-demo
```

### Code generator changes

```sh
cd codegen && cargo run --bin lepticons-codegen && cd .. && cargo clippy -p lepticons --all-targets -- -D warnings && cargo test -p lepticons
```

Always run the generator after modifying it -- the output (`lucide_icon_data.rs`) must compile and pass clippy.

### Demo dev server

```sh
cd demo && trunk serve  # localhost:8080
```

## Dependency Chains (when X changes, also do Y)

- `codegen/` changed -> re-run generator -> verify lepticons compiles
- `lucide/` submodule updated -> re-run generator -> verify lepticons compiles
- `lepticons/src/lib.rs` props changed -> update `lepticons/README.md` props table -> verify demo still compiles
- `lepticons/Cargo.toml` features changed -> update `lepticons/README.md` categories list
- `demo/style/input.css` changed -> trunk rebuilds automatically in `trunk serve`
- `demo/tailwind.config.js` changed -> restart `trunk serve`

## Design Decisions (do not regress)

- **Strum for icon metadata, not a proc macro**: strum derives are stable, well-tested, and give us `EnumIter` + `EnumProperty` for free. A custom proc macro would duplicate this work.
- **Category features, not per-icon features**: 42 categories vs 706 per-icon features. Per-icon is impractical for Cargo.toml and user ergonomics. Category grouping is semantically meaningful.
- **`OnceLock` for search index and categories cache**: zero per-call allocation. Built once on first use. Thread-safe for SSR. No `lazy_static` dependency needed (std only).
- **`inner_html` for SVG content**: Leptos renders this correctly in both CSR and SSR. Avoids generating individual Leptos view nodes per SVG element (which would bloat the component tree for 700+ icons).
- **`&'static str` from strum props**: zero allocation. Never convert to `String` unless the caller needs ownership. The `svg()`, `tags_str()`, `categories_str()` methods return `&'static str`.
- **No rendering mode features on the library**: the library is rendering-mode agnostic. Only the consuming app enables `csr`/`ssr`/`hydrate` on leptos.
- **Decimal truncation in code generator**: SVG path coordinates are truncated to 2 decimal places to reduce generated file size without visible quality loss.

## Conventions

### Rust

- Leptos 0.8.x -- use `leptos::prelude::*`
- Props: `#[prop(into)]` for signals, `#[prop(into, optional)]` for optional values
- Document all `pub` items with `///` doc comments, including every component prop
- Prefer `&'static str` over `String` in library APIs where data comes from strum props
- Feature gates: `#[cfg(any(feature = "cat1", feature = "cat2"))]` per icon variant

### Styling (demo app)

- Tailwind CSS with shadcn-inspired HSL color system (CSS variables in `demo/style/input.css`)
- Dark mode: `class` strategy on `<html>`, persisted in localStorage
- Accent: orange-700 (`#ea580c`) for selection, tooltips, highlights
- Use semantic color names (`primary`, `secondary`, `background`) not raw colors

### Commits

Conventional commits, imperative mood:

```
feat: add reactive props to Icon component
fix: correct search index word boundary matching
refactor: extract IconGrid from demo into picker crate
docs: update README props table
chore: bump leptos to 0.8.17
ci: add WASM build check to CI
```

Types: `feat`, `fix`, `refactor`, `docs`, `chore`, `ci`, `test`, `perf`, `style`

Scope is optional: `feat(picker): add IconPickerPopover component`

## Pitfalls

- **`lucide_icon_data.rs` is generated** -- never edit it. Run `codegen` to regenerate.
- **`lucide/` is a git submodule** -- use `git clone --recurse-submodules` or `git submodule update --init`. CI uses `actions/checkout` with `submodules: true`.
- **Demo clippy vs library clippy** -- the library crate is clean. Demo app may have warnings. Always check library independently with `cargo clippy -p lepticons`.
- **WASM target required for demo** -- `rustup target add wasm32-unknown-unknown` before building the demo.
- **Trunk asset handling** -- `demo/index.html` uses `data-trunk` link tags for assets. Static files go in `demo/public/`. Trunk copies them to `demo/dist/` on build.
- **Strum prop values are comma-separated strings** -- `tags` and `categories` are stored as `"tag1,tag2,tag3"` in strum props. The `tags()` and `categories()` methods split and trim them into iterators.

## Git Workflow

- `develop`: default working branch. PRs target here.
- `master`: production. Push triggers Vercel deploy. Do not push directly.
- Tags (`v*`): trigger crates.io publish. Tag must match `lepticons/Cargo.toml` version.
- Automated weekly PR updates `lucide/` submodule every Monday 9am UTC.

## Publishing Checklist

1. Bump version in `lepticons/Cargo.toml`
2. Update compatibility table in `lepticons/README.md` if Leptos version changed
3. Merge to master
4. Tag: `git tag v0.X.0 && git push origin v0.X.0`
5. Publish workflow verifies version match, publishes to crates.io, creates GitHub release
