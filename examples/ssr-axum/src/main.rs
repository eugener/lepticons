//! Minimal Axum + Leptos SSR example demonstrating that lepticons icons
//! render correctly in server-rendered HTML, with no client-side WASM.
//!
//! Run with:
//!     cargo run
//!
//! Then visit http://localhost:3000 and view the page source. Each `<Icon>`
//! and `<CustomIcon>` is emitted as inline `<svg>...</svg>` markup directly
//! in the HTTP response -- no JavaScript or WASM required to display icons.

use axum::{response::Html, routing::get, Router};
use leptos::prelude::*;
use lepticons::{CustomIcon, Icon, LucideGlyph};

const COMPANY_LOGO: &str = r#"<path d="M12 2L2 7l10 5 10-5-10-5z" />"#;

#[component]
fn Page() -> impl IntoView {
    view! {
        <html>
            <head>
                <meta charset="utf-8"/>
                <title>"Lepticons SSR demo"</title>
                <style>
                    "body { font-family: system-ui, sans-serif; max-width: 720px; margin: 2rem auto; padding: 0 1rem; color: #1a1a1a; }
                     h1 { margin-bottom: 0.25rem; }
                     p.lead { color: #666; margin-top: 0; }
                     .row { display: flex; gap: 1rem; align-items: center; padding: 0.75rem 0; border-bottom: 1px solid #eee; }
                     .row svg { color: #1a1a1a; flex-none: 0; }
                     .row code { font-size: 0.875rem; color: #666; }
                     .custom { color: #c03a17; }"
                </style>
            </head>
            <body>
                <h1>"Lepticons SSR demo"</h1>
                <p class="lead">
                    "These icons are rendered on the server and embedded as inline SVG. "
                    "No client-side WASM, no hydration, no JavaScript."
                </p>

                <div class="row">
                    <Icon glyph=LucideGlyph::Search />
                    <code>"<Icon glyph=LucideGlyph::Search />"</code>
                </div>
                <div class="row">
                    <Icon glyph=LucideGlyph::Heart stroke="#c03a17" />
                    <code>"<Icon glyph=LucideGlyph::Heart stroke=\"#c03a17\" />"</code>
                </div>
                <div class="row">
                    <Icon glyph=LucideGlyph::Bell size="32" />
                    <code>"<Icon glyph=LucideGlyph::Bell size=\"32\" />"</code>
                </div>
                <div class="row custom">
                    <CustomIcon
                        svg=COMPANY_LOGO
                        fill="currentColor"
                        stroke="none"
                    />
                    <code>"<CustomIcon svg=COMPANY_LOGO fill=\"currentColor\" stroke=\"none\" />"</code>
                </div>

                <p class="lead" style="margin-top:2rem">
                    "View page source (Cmd/Ctrl+U) to confirm each icon is full inline SVG markup."
                </p>
            </body>
        </html>
    }
}

async fn render() -> Html<String> {
    let owner = Owner::new();
    let body = owner.with(|| Page().to_html());
    Html(format!("<!DOCTYPE html>{}", body))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(render));
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Lepticons SSR demo: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
