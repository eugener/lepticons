//! Demo-only DOM helpers: clipboard flash, color sniffing, and blob/PNG
//! downloads. Pulled out of `icons_view.rs` to keep that file focused on
//! view code.

use leptos::prelude::*;
use leptos::wasm_bindgen::{self, JsCast};
use web_sys::js_sys;

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

/// Triggers a browser download of `content` as `filename` with the given
/// MIME type. No-ops outside a browser context.
pub fn download_blob(content: &str, filename: &str, mime: &str) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else {
        return;
    };
    let Some(body) = document.body() else { return };
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(content));
    let opts = web_sys::BlobPropertyBag::new();
    opts.set_type(mime);
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &opts) else {
        return;
    };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else {
        return;
    };
    let Ok(el) = document.create_element("a") else {
        return;
    };
    let anchor: web_sys::HtmlAnchorElement = el.unchecked_into();
    anchor.set_href(&url);
    anchor.set_download(filename);
    let _ = body.append_child(&anchor);
    anchor.click();
    let _ = body.remove_child(&anchor);
    let _ = web_sys::Url::revoke_object_url(&url);
}

/// Renders `svg_str` to a 256-pixel PNG via an off-DOM `<canvas>` and
/// triggers a download named `<name>.png`. No-ops outside a browser
/// context.
pub fn download_png(svg_str: &str, name: &str) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else {
        return;
    };
    let Some(body) = document.body() else { return };

    let Ok(canvas_el) = document.create_element("canvas") else {
        return;
    };
    let canvas: web_sys::HtmlCanvasElement = canvas_el.unchecked_into();
    let png_size = 256;
    canvas.set_width(png_size);
    canvas.set_height(png_size);

    let Ok(ctx_val) = canvas.get_context("2d") else {
        return;
    };
    let Some(ctx_obj) = ctx_val else { return };
    let ctx: web_sys::CanvasRenderingContext2d =
        ctx_obj.unchecked_into::<web_sys::CanvasRenderingContext2d>();

    let Ok(img) = web_sys::HtmlImageElement::new() else {
        return;
    };
    let data_url = format!(
        "data:image/svg+xml,{}",
        js_sys::encode_uri_component(svg_str)
    );

    let filename = format!("{}.png", name);
    let canvas_clone = canvas.clone();
    let img_clone = img.clone();
    let body_clone = body;

    let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
        let Ok(()) = ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &img_clone,
            0.0,
            0.0,
            png_size as f64,
            png_size as f64,
        ) else {
            return;
        };
        if let Ok(png_url) = canvas_clone.to_data_url_with_type("image/png") {
            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            let Ok(el) = document.create_element("a") else {
                return;
            };
            let anchor: web_sys::HtmlAnchorElement = el.unchecked_into();
            anchor.set_href(&png_url);
            anchor.set_download(&filename);
            let _ = body_clone.append_child(&anchor);
            anchor.click();
            let _ = body_clone.remove_child(&anchor);
        }
    });
    img.set_onload(Some(cb.as_ref().unchecked_ref()));
    img.set_src(&data_url);
}
