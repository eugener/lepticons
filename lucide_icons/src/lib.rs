mod generated_icons;
pub use generated_icons::*;
use leptos::*;


const DEFAULT_SIZE: u16 = 24;
const DEFAULT_FILL: &str = "none";
const DEFAULT_STROKE: &str = "black";
const DEFAULT_STROKE_WIDTH: u16 = 2;


#[derive(Copy, Clone, Debug)]
pub struct IconType<'a> {
    pub content: &'a str,
}

# [component]
pub fn Icon<'a>(
    cx: Scope,
    kind: IconType<'a>,
    #[prop(default = DEFAULT_SIZE)]
    size: u16,
    #[prop(default = DEFAULT_FILL.to_string())]
    fill: String,
    #[prop(default = DEFAULT_STROKE.to_string())]
    stroke: String,
    #[prop(default = DEFAULT_STROKE_WIDTH)]
    stroke_width: u16,
) -> impl IntoView {

    view! { cx,
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width  =format!("{}", size)
              height =format!("{}", size)
              viewBox=format!("0 0 {} {}", size, size)
              fill={fill}
              stroke={stroke}
              stroke-width=format!("{}", stroke_width)
              stroke-linecap="round"
              stroke-linejoin="round"
              inner_html={kind.content.to_string()}
            />
        }
}


