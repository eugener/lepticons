//! Demo-only DOM helpers: clipboard flash and computed-color sniffing.
//! Pulled out of `icons_view.rs` to keep that file focused on view code.
//! Download helpers live in `lepticons_picker::download`.

use leptos::prelude::*;

/// Copies text to clipboard, flashes a "copied" signal for 2 seconds, and
/// closes a menu. Wraps [`lepticons_picker::copy_to_clipboard`] with the
/// UI-state side-effects the icon detail drawer needs.
pub fn copy_and_flash(
    text: &str,
    set_copied: WriteSignal<bool>,
    set_menu_open: WriteSignal<bool>,
) {
    lepticons_picker::copy_to_clipboard(text);
    set_copied.set(true);
    set_menu_open.set(false);
    set_timeout(
        move || set_copied.set(false),
        std::time::Duration::from_secs(2),
    );
}

/// Returns the computed text color of `<body>` as a hex string
/// (e.g. `"#1a1a2e"`). Falls back to `"#000000"` when any DOM lookup
/// fails.
pub fn current_text_color_hex() -> String {
    let fallback = "#000000".to_string();
    let Some(window) = web_sys::window() else {
        return fallback;
    };
    let Some(document) = window.document() else {
        return fallback;
    };
    let Some(body) = document.body() else {
        return fallback;
    };
    let Ok(computed) = window.get_computed_style(&body) else {
        return fallback;
    };
    let Some(style) = computed else { return fallback };
    let Ok(rgb) = style.get_property_value("color") else {
        return fallback;
    };
    rgb_to_hex(&rgb).unwrap_or(fallback)
}

/// Converts `"rgb(r, g, b)"` or `"rgba(r, g, b, a)"` to `"#rrggbb"`.
fn rgb_to_hex(rgb: &str) -> Option<String> {
    let inner = rgb
        .trim()
        .strip_prefix("rgb")?
        .trim_start_matches('a')
        .strip_prefix('(')?
        .strip_suffix(')')?;
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() < 3 {
        return None;
    }
    let r: u8 = parts[0].trim().parse().ok()?;
    let g: u8 = parts[1].trim().parse().ok()?;
    let b: u8 = parts[2].trim().parse().ok()?;
    Some(format!("#{:02x}{:02x}{:02x}", r, g, b))
}

