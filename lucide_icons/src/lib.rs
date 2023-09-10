mod generated_icons;
pub use generated_icons::*;

use leptos::*;


// const DEFAULT_SIZE: u16 = 24;
// const DEFAULT_FILL: &str = "none";
// const DEFAULT_STROKE: &str = "black";
// const DEFAULT_STROKE_WIDTH: u16 = 2;
//
// # [component]
// pub fn LucideIcon(
//     cx: Scope,
//     #[prop(default = DEFAULT_SIZE)]
//     size: u16,
//     #[prop(default = DEFAULT_FILL.to_string())]
//     fill: String,
//     #[prop(default = DEFAULT_STROKE.to_string())]
//     stroke: String,
//     #[prop(default = DEFAULT_STROKE_WIDTH)]
//     stroke_width: u16,
// ) -> impl IntoView {
//
//     view! { cx,
//             <svg
//               xmlns="http://www.w3.org/2000/svg"
//               width  =format!("{}", size)
//               height =format!("{}", size)
//               viewBox=format!("0 0 {} {}", size, size)
//               fill={fill}
//               stroke={stroke}
//               stroke-width=format!("{}", stroke_width)
//               stroke-linecap="round"
//               stroke-linejoin="round"
//             >
//               <circle cx="16" cy="4" r="1" />
//               <path d="m18 19 1-7-6 1" />
//               <path d="m5 8 3-3 5.5 3-2.36 3.5" />
//               <path d="M4.24 14.5a5 5 0 0 0 6.88 6" />
//               <path d="M13.76 17.5a5 5 0 0 0-6.88-6" />
//             </svg>
//         }
// }