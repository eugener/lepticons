# Rendering Modes

Lepticons works with every Leptos rendering mode. The library is rendering-mode agnostic -- it has no `csr`/`ssr` feature flags, no runtime checks, and no DOM dependencies in the core `Icon` and `CustomIcon` components.

This doc covers the four common setups and the two caveats worth knowing about.

---

## CSR (Client-Side Rendering)

The default Trunk-based setup. The browser receives an empty HTML shell, downloads the WASM bundle, and renders everything client-side.

```toml
# Cargo.toml
[dependencies]
leptos = "0.8"
lepticons = "0.10"
```

```rust
use leptos::prelude::*;
use lepticons::{Icon, LucideGlyph};

#[component]
fn App() -> impl IntoView {
    view! { <Icon glyph=LucideGlyph::Search /> }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
```

```sh
trunk serve
```

That's it. No additional configuration.

---

## SSR + Hydration

The server pre-renders HTML on each request, ships the markup with embedded state, and the client hydrates the existing DOM.

Lepticons icons render identically in both phases because `inner_html` round-trips cleanly through hydration.

```toml
[dependencies]
leptos = { version = "0.8", features = ["ssr", "hydrate"] }
lepticons = "0.10"
axum = "0.7"
tokio = { version = "1", features = ["full"] }
leptos_axum = "0.8"
```

```rust
// On the server
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use lepticons::{Icon, LucideGlyph};

#[component]
fn App() -> impl IntoView {
    view! { <Icon glyph=LucideGlyph::Search /> }
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes(&conf.leptos_options, routes, App)
        .with_state(conf.leptos_options);
    // ...bind and serve
}
```

The icon SVG is emitted as part of the server-rendered HTML. The client hydration pass attaches reactive handlers without re-rendering the SVG markup.

---

## Static Site Generation

For fully-static export, use Leptos `0.8`'s static SSR. Icons compile into the output HTML at build time -- zero runtime cost.

```rust
// Same App component as above, rendered once to a string
let html = leptos::prelude::ssr::render_to_string(App);
std::fs::write("dist/index.html", html.to_string()).unwrap();
```

Useful for landing pages, docs sites, or any content where reactivity is not needed.

---

## Islands / Selective Hydration

Lepticons icons in non-interactive zones can stay as static SSR output, never hydrating. Icons inside an `#[island]` component participate in hydration normally. No lepticons-specific configuration is needed.

```rust
use leptos::prelude::*;
use lepticons::{Icon, LucideGlyph};

#[island]
fn ToggleButton() -> impl IntoView {
    let (active, set_active) = signal(false);
    view! {
        <button on:click=move |_| set_active.update(|a| *a = !*a)>
            <Icon glyph=move || if active.get() { LucideGlyph::Sun } else { LucideGlyph::Moon } />
        </button>
    }
}
```

The icon inside the island reacts to the local signal; surrounding static `<Icon>` calls render once on the server and stay frozen.

---

## Caveats

### `DrawIcon` requires hydration

`lepticons-animate::DrawIcon` reaches into the DOM after mount to call `getTotalLength()` on each `<path>` element. In SSR mode, the initial server-rendered HTML shows the icon at its final state (paths fully drawn) until hydration completes -- at which point the draw-in animation runs.

If pre-hydration appearance matters for your use case, conditionally render `DrawIcon` only after a `Suspense` boundary or behind an `#[island]`.

```rust
view! {
    <Suspense fallback=|| view! { <Icon glyph=LucideGlyph::Check /> }>
        <DrawIcon glyph=LucideGlyph::Check duration_ms=500 />
    </Suspense>
}
```

### `IconPicker` requires hydration

`lepticons-picker` components rely on signals (search state, selected glyph, focus index) and click/key event handlers. They render usable static markup on the server, but search and selection don't work until hydration completes.

For most apps this is the desired behavior. For static-only sites, omit the picker.

### `inner_html` and Content Security Policy

Lepticons sets icon content via `inner_html` on the wrapping `<svg>` element. If your app uses a strict Content-Security-Policy that disallows inline content, the icons will still render -- `inner_html` for SVG paths is not subject to `script-src` or `style-src` directives. No CSP changes are needed for lepticons.

---

## Summary

| Mode | Lepticons | DrawIcon | IconPicker |
|------|-----------|----------|------------|
| CSR | works | works | works |
| SSR + hydrate | works | works post-hydration | works post-hydration |
| Static SSG | works | renders final state, no animation | omit |
| Islands | works | works inside islands | works inside islands |

The `Icon` and `CustomIcon` components have no rendering-mode requirements. Everything reactive (animations, picker, signal-driven props) follows Leptos's standard hydration rules.
