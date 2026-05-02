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

const SHOWCASE: &[(LucideGlyph, &str)] = &[
    (LucideGlyph::Search, "Search"),
    (LucideGlyph::Heart, "Heart"),
    (LucideGlyph::Bell, "Bell"),
    (LucideGlyph::Settings, "Settings"),
    (LucideGlyph::User, "User"),
    (LucideGlyph::House, "House"),
    (LucideGlyph::Star, "Star"),
    (LucideGlyph::Mail, "Mail"),
];

const STYLES: &str = r#"
:root {
  --bg: #fafaf7;
  --surface: #ffffff;
  --ink: #1a1a1a;
  --muted: #6b6b6b;
  --line: #e8e6df;
  --accent: #c03a17;
  --accent-soft: rgba(192, 58, 23, 0.08);
  --code-bg: #1f1d1a;
  --code-ink: #e8e6df;
  --radius: 12px;
}
* { box-sizing: border-box; }
body {
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
  background: var(--bg);
  color: var(--ink);
  margin: 0;
  line-height: 1.6;
  -webkit-font-smoothing: antialiased;
}
.shell { max-width: 880px; margin: 0 auto; padding: 4rem 1.5rem 6rem; }
header { margin-bottom: 3rem; }
.brand {
  display: inline-flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.375rem 0.875rem;
  background: var(--accent-soft);
  color: var(--accent);
  border-radius: 999px;
  font-size: 0.8125rem;
  font-weight: 500;
  letter-spacing: 0.02em;
}
h1 {
  font-size: 2.5rem;
  font-weight: 600;
  letter-spacing: -0.02em;
  margin: 1.25rem 0 0.5rem;
}
.lead { font-size: 1.125rem; color: var(--muted); margin: 0; max-width: 60ch; }
.lead strong { color: var(--ink); font-weight: 600; }
section { margin-top: 3rem; }
section h2 {
  font-size: 0.8125rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--muted);
  margin: 0 0 1rem;
}
.card {
  background: var(--surface);
  border: 1px solid var(--line);
  border-radius: var(--radius);
  padding: 1.75rem;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 1.25rem;
}
.cell {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem 0.5rem;
  border-radius: 8px;
  transition: background 0.15s;
}
.cell:hover { background: var(--accent-soft); color: var(--accent); }
.cell svg { width: 24px; height: 24px; }
.cell .name { font-size: 0.8125rem; color: var(--muted); }
.cell:hover .name { color: var(--accent); }
.sizes {
  display: flex;
  align-items: flex-end;
  gap: 2rem;
  flex-wrap: wrap;
}
.size-item { display: flex; flex-direction: column; align-items: center; gap: 0.5rem; }
.size-item .label { font-size: 0.75rem; color: var(--muted); font-family: ui-monospace, monospace; }
.colors { display: flex; gap: 1.5rem; flex-wrap: wrap; align-items: center; }
.brand-card { display: flex; align-items: center; gap: 1.5rem; }
.brand-card svg { color: var(--accent); }
pre {
  background: var(--code-bg);
  color: var(--code-ink);
  padding: 1rem 1.25rem;
  border-radius: 8px;
  margin: 1rem 0 0;
  overflow-x: auto;
  font-size: 0.8125rem;
  line-height: 1.55;
  font-family: ui-monospace, 'SF Mono', Menlo, monospace;
}
pre .keyword { color: #c1a5ff; }
pre .type { color: #ffb86c; }
pre .string { color: #a8e07a; }
pre .comment { color: #6b6b6b; font-style: italic; }
.footer {
  margin-top: 5rem;
  padding-top: 2rem;
  border-top: 1px solid var(--line);
  display: flex;
  gap: 1.5rem;
  font-size: 0.875rem;
  color: var(--muted);
}
.footer a { color: var(--muted); text-decoration: none; }
.footer a:hover { color: var(--accent); }
.callout {
  margin-top: 2rem;
  padding: 1rem 1.25rem;
  background: var(--accent-soft);
  border-left: 3px solid var(--accent);
  border-radius: 0 8px 8px 0;
  font-size: 0.9375rem;
  color: var(--ink);
}
.callout code {
  background: var(--surface);
  padding: 0.125rem 0.375rem;
  border-radius: 4px;
  font-size: 0.8125rem;
  border: 1px solid var(--line);
}
"#;

#[component]
fn Page() -> impl IntoView {
    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"Lepticons SSR demo"</title>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
                <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap"/>
                <style>{STYLES}</style>
            </head>
            <body>
                <div class="shell">
                    <header>
                        <span class="brand">
                            <Icon glyph=LucideGlyph::Sparkles size="14" />
                            "Lepticons SSR"
                        </span>
                        <h1>"Server-rendered Lucide icons"</h1>
                        <p class="lead">
                            "Every icon on this page is "
                            <strong>"plain inline SVG"</strong>
                            " emitted by the Axum server. No WASM, no hydration, no JavaScript. "
                            "View page source to confirm."
                        </p>
                    </header>

                    <section>
                        <h2>"Showcase"</h2>
                        <div class="card">
                            <div class="grid">
                                {SHOWCASE.iter().map(|(glyph, name)| view! {
                                    <div class="cell">
                                        <Icon glyph=*glyph />
                                        <span class="name">{*name}</span>
                                    </div>
                                }).collect_view()}
                            </div>
                        </div>
                    </section>

                    <section>
                        <h2>"Sizes"</h2>
                        <div class="card">
                            <div class="sizes">
                                <div class="size-item">
                                    <Icon glyph=LucideGlyph::Compass size="16" />
                                    <span class="label">"16"</span>
                                </div>
                                <div class="size-item">
                                    <Icon glyph=LucideGlyph::Compass size="24" />
                                    <span class="label">"24"</span>
                                </div>
                                <div class="size-item">
                                    <Icon glyph=LucideGlyph::Compass size="32" />
                                    <span class="label">"32"</span>
                                </div>
                                <div class="size-item">
                                    <Icon glyph=LucideGlyph::Compass size="48" />
                                    <span class="label">"48"</span>
                                </div>
                                <div class="size-item">
                                    <Icon glyph=LucideGlyph::Compass size="72" />
                                    <span class="label">"72"</span>
                                </div>
                            </div>
                            <pre>{"<Icon glyph=LucideGlyph::Compass size=\"48\" />"}</pre>
                        </div>
                    </section>

                    <section>
                        <h2>"Colors"</h2>
                        <div class="card">
                            <div class="colors">
                                <Icon glyph=LucideGlyph::Heart stroke="#c03a17" size="32" />
                                <Icon glyph=LucideGlyph::Heart stroke="#2563eb" size="32" />
                                <Icon glyph=LucideGlyph::Heart stroke="#16a34a" size="32" />
                                <Icon glyph=LucideGlyph::Heart stroke="#a855f7" size="32" />
                                <Icon glyph=LucideGlyph::Heart stroke="#f59e0b" size="32" />
                            </div>
                            <pre>{"<Icon glyph=LucideGlyph::Heart stroke=\"#c03a17\" size=\"32\" />"}</pre>
                        </div>
                    </section>

                    <section>
                        <h2>"CustomIcon"</h2>
                        <div class="card">
                            <div class="brand-card">
                                <CustomIcon
                                    svg=COMPANY_LOGO
                                    fill="currentColor"
                                    stroke="none"
                                    size="48"
                                />
                                <div>
                                    <div style="font-weight:600;">"For brand logos and in-house icons"</div>
                                    <div style="color:var(--muted);font-size:0.9375rem;">
                                        "Lucide intentionally drops brand logos. CustomIcon takes inline SVG markup with the same prop API."
                                    </div>
                                </div>
                            </div>
                            <pre>{"const COMPANY_LOGO: &str = r#\"<path d=\\\"M12 2L2 7l10 5 10-5-10-5z\\\" />\"#;\n\n<CustomIcon svg=COMPANY_LOGO fill=\"currentColor\" stroke=\"none\" size=\"48\" />"}</pre>
                        </div>
                    </section>

                    <div class="callout">
                        "Right-click anywhere on the page and pick "
                        <code>"View Page Source"</code>
                        ". You will see complete "
                        <code>"<svg>...</svg>"</code>
                        " markup for every icon -- no script tag, no hydration scaffolding, no client-side runtime."
                    </div>

                    <div class="footer">
                        <a href="https://lepticons.9bits.cc/">"Demo"</a>
                        <a href="https://github.com/eugener/lepticons">"GitHub"</a>
                        <a href="https://crates.io/crates/lepticons">"crates.io"</a>
                        <a href="https://github.com/eugener/lepticons/blob/develop/docs/rendering-modes.md">"Rendering modes"</a>
                    </div>
                </div>
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
