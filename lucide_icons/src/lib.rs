mod lucide_icon_data;
mod lucide_icon_impl;

pub use lucide_icon_impl::Glyph;
pub use lucide_icon_data::LucideGlyph;

use leptos::*;
// use leptos::logging::log;

const DEFAULT_SIZE: u16 = 24;
const DEFAULT_FILL: &'static str = "none";
const DEFAULT_STROKE: &'static str = "currentColor";
const DEFAULT_STROKE_WIDTH: f32 = 1.5;

// TODO cache decompressed svg


#[component]
pub fn Icon<T: Glyph>(
    icon: T,
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = DEFAULT_SIZE)]
    size: u16,
    #[prop(default = DEFAULT_FILL)]
    fill: &'static str,
    #[prop(default = DEFAULT_STROKE)]
    stroke: &'static str,
    #[prop(default = DEFAULT_STROKE_WIDTH)]
    stroke_width: f32,
) -> impl IntoView {

    // log!("Icon: {}", icon.svg());

    view! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class = class.to_string()
          width  =format!("{}", size)
          height =format!("{}", size)
          viewBox=format!("0 0 {} {}", size, size)
          fill={fill}
          stroke={stroke}
          stroke-width=format!("{}", stroke_width)
          stroke-linecap="round"
          stroke-linejoin="round"
          inner_html={icon.svg()}
        />
    }
}


