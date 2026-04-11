use leptos::prelude::*;
use leptos::text_prop::TextProp;
use lepticons::{Glyph, LucideGlyph};
use wasm_bindgen::JsCast;

use crate::Easing;

/// Renders an icon with a stroke draw-in animation.
///
/// The icon's paths animate from invisible to fully drawn over `duration_ms`.
///
/// # Example
///
/// ```rust,ignore
/// use lepticons_animate::{DrawIcon, Easing};
/// use lepticons::LucideGlyph;
///
/// // Default easing (ease-in-out, 600ms)
/// <DrawIcon glyph=LucideGlyph::Check />
///
/// // Custom duration and easing
/// <DrawIcon glyph=LucideGlyph::Heart duration_ms=800 easing=Easing::EaseOut />
/// ```
#[component]
pub fn DrawIcon(
    /// The icon to render.
    #[prop(into)]
    glyph: Signal<LucideGlyph>,
    /// Animation duration in milliseconds (default: 600).
    #[prop(default = 600)]
    duration_ms: u32,
    /// Delay before animation starts in milliseconds (default: 0).
    #[prop(default = 0)]
    delay_ms: u32,
    /// Transition timing function (default: `Easing::EaseInOut`).
    #[prop(default = Easing::EaseInOut)]
    easing: Easing,
    /// CSS class for the outer wrapper.
    #[prop(into, optional)]
    class: Option<TextProp>,
    /// Width and height in pixels (default: "24").
    #[prop(into, optional)]
    size: Option<TextProp>,
    /// SVG fill color (default: "none").
    #[prop(into, optional)]
    fill: Option<TextProp>,
    /// SVG stroke color (default: "currentColor").
    #[prop(into, optional)]
    stroke: Option<TextProp>,
    /// SVG stroke width (default: "1.5").
    #[prop(into, optional)]
    stroke_width: Option<TextProp>,
) -> impl IntoView {
    let size = size.unwrap_or_else(|| "24".into());
    let size2 = size.clone();
    let fill = fill.unwrap_or_else(|| "none".into());
    let stroke = stroke.unwrap_or_else(|| "currentColor".into());
    let stroke_width = stroke_width.unwrap_or_else(|| "1.5".into());
    let easing_css = easing.as_css();

    let wrapper_ref = NodeRef::<leptos::html::Div>::new();

    // On mount/glyph change, find all geometry elements and animate stroke-dashoffset.
    // Deferred to next frame so inner_html has populated the SVG children.
    Effect::new(move |_| {
        glyph.get();

        let Some(wrapper) = wrapper_ref.get() else { return };
        let wrapper_el: web_sys::HtmlElement = (*wrapper).clone();

        // First frame: wait for inner_html to populate SVG children
        request_animation_frame(move || {
            let wrapper_as_el: &web_sys::Element = wrapper_el.as_ref();
            let Some(svg) = wrapper_as_el.first_element_child() else { return };
            let children = svg.children();
            let count = children.length();

            for i in 0..count {
                let Some(child) = children.item(i) else { continue };

                let Ok(geom) = child.clone().dyn_into::<web_sys::SvgGeometryElement>() else {
                    continue;
                };
                let length: f32 = geom.get_total_length();
                if length <= 0.0 {
                    continue;
                }

                let svg_child: web_sys::SvgElement = child.unchecked_into();
                let s = svg_child.style();

                let len_str = length.to_string();
                // Set initial state: fully hidden
                let _ = s.set_property("stroke-dasharray", &len_str);
                let _ = s.set_property("stroke-dashoffset", &len_str);
                let _ = s.set_property("transition", "none");

                // Second frame: enable transition and animate to visible
                let s_clone = s.clone();
                request_animation_frame(move || {
                    let _ = s_clone.set_property(
                        "transition",
                        &format!(
                            "stroke-dashoffset {}ms {} {}ms",
                            duration_ms, easing_css, delay_ms
                        ),
                    );
                    let _ = s_clone.set_property("stroke-dashoffset", "0");
                });
            }
        });
    });

    view! {
        <div node_ref=wrapper_ref
             class=move || class.as_ref().map(|c| c.get().to_string()).unwrap_or_default()
             style="display:inline-block;line-height:0">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width=move || size.get()
                height=move || size2.get()
                viewBox="0 0 24 24"
                fill=move || fill.get()
                stroke=move || stroke.get()
                stroke-width=move || stroke_width.get()
                stroke-linecap="round"
                stroke-linejoin="round"
                inner_html=move || glyph.get().svg()
            />
        </div>
    }
}
