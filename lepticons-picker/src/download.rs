//! Browser download helpers for icons. Best-effort wrappers around
//! `Blob` + `<a download>` and `<canvas>` + `to_data_url`. All functions
//! no-op outside a browser context.

use leptos::wasm_bindgen::{self, JsCast};
use lepticons::LucideGlyph;
use web_sys::js_sys;

use crate::copy::IconCopyFormat;

/// Triggers a browser download of `content` as `filename` with the given
/// MIME type. Generic helper; prefer [`download_svg`] / [`download_png`]
/// for the icon-specific cases.
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

/// Downloads `icon` as an SVG file. The filename defaults to
/// `<kebab-name>.svg` (e.g. `arrow-right.svg`) when `filename` is `None`.
pub fn download_svg(icon: LucideGlyph, filename: Option<&str>) {
    let default;
    let name = match filename {
        Some(n) => n,
        None => {
            default = format!("{}.svg", icon.kebab_name());
            default.as_str()
        }
    };
    download_blob(&IconCopyFormat::Svg.render(icon), name, "image/svg+xml");
}

/// Rasterizes `icon` to a `size`-pixel PNG via an off-DOM `<canvas>` and
/// triggers a download. Filename defaults to `<kebab-name>.png` when
/// `filename` is `None`. The render is asynchronous (waits for the
/// `<img>` load event); this function returns immediately.
pub fn download_png(icon: LucideGlyph, filename: Option<&str>, size: u32) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else {
        return;
    };
    let Some(body) = document.body() else { return };

    let Ok(canvas_el) = document.create_element("canvas") else {
        return;
    };
    let canvas: web_sys::HtmlCanvasElement = canvas_el.unchecked_into();
    canvas.set_width(size);
    canvas.set_height(size);

    let Ok(ctx_val) = canvas.get_context("2d") else {
        return;
    };
    let Some(ctx_obj) = ctx_val else { return };
    let ctx: web_sys::CanvasRenderingContext2d =
        ctx_obj.unchecked_into::<web_sys::CanvasRenderingContext2d>();

    let Ok(img) = web_sys::HtmlImageElement::new() else {
        return;
    };
    let data_url = IconCopyFormat::DataUrl.render(icon);

    let filename = filename
        .map(str::to_string)
        .unwrap_or_else(|| format!("{}.png", icon.kebab_name()));
    let canvas_clone = canvas.clone();
    let img_clone = img.clone();
    let body_clone = body;

    let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
        let Ok(()) = ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &img_clone,
            0.0,
            0.0,
            size as f64,
            size as f64,
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
