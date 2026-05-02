# Lepticons SSR with Axum

Minimal example showing that lepticons icons render correctly on the server and ship to the browser as inline SVG -- no client-side WASM, no hydration, no JavaScript.

This is a standalone example. It is intentionally not part of the lepticons workspace so you can copy the directory into your own project as a starting point.

## Run

```sh
cd examples/ssr-axum
cargo run
```

Visit `http://localhost:3000` and view the page source (Cmd/Ctrl+U). Each icon appears as a full `<svg>...</svg>` block directly in the HTML response.

## What it demonstrates

- `Icon` and `CustomIcon` from `lepticons` work on the server with no special configuration
- The `inner_html` SVG content round-trips cleanly through Leptos `to_html()`
- Custom props (`size`, `stroke`, `fill`) flow through to the rendered SVG attributes
- Server response is plain HTML that any browser displays without WASM

## What it does NOT do

- No hydration. This is pure server-side rendering. The page is static.
- No `IconPicker` or `DrawIcon`. Those require client-side reactivity (signals, DOM access).

If you need interactivity, add `leptos = { version = "0.8", features = ["ssr", "hydrate"] }`, set up a client-side WASM build with `cargo-leptos`, and use `leptos_axum::LeptosRoutes` instead of the manual `to_html()` call in this example. See [`docs/rendering-modes.md`](../../docs/rendering-modes.md) for the broader rendering-mode story.

## Cargo.toml

When copying this example into your own project, replace the path dep with the published crate:

```toml
lepticons = "0.12"
```
