mod lucide_icon_data;
mod lucide_icon_impl;

pub use lucide_icon_data::LucideGlyph;
pub use lucide_icon_impl::Glyph;

use leptos::Signal;
use leptos::*;
// use leptos::logging::log;

const DEFAULT_SIZE: u16 = 24;
const DEFAULT_FILL: &'static str = "none";
const DEFAULT_STROKE: &'static str = "currentColor";
const DEFAULT_STROKE_WIDTH: f32 = 1.5;

// TODO cache decompressed svg

#[component]
pub fn Icon<T: Glyph + 'static>(
    #[prop(into)] glyph: Signal<T>,
    #[prop(into, default = Signal::from(""))] class: Signal<&'static str>,
    #[prop(into, default = Signal::from(DEFAULT_SIZE))] size: Signal<u16>,
    #[prop(into, default = Signal::from(DEFAULT_FILL))] fill: Signal<&'static str>,
    #[prop(into, default = Signal::from(DEFAULT_STROKE))] stroke: Signal<&'static str>,
    #[prop(into, default = Signal::from(DEFAULT_STROKE_WIDTH))] stroke_width: Signal<f32>,
) -> impl IntoView {
    // log!("Icon: {}", icon.svg());

    // let svg = create_memo(move |_| glyph.get().svg());
    // let glyph = glyph.get();
    // let size = size.get();

    view! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class   = move || class.get()
          width   = move || format!("{}", size.get())
          height  = move || format!("{}", size.get())
          viewBox = move || format!("0 0 {} {}", size.get(), size.get())
          fill    = fill.get()
          stroke  = stroke.get()
          stroke-width    = move || format!("{}", stroke_width.get())
          stroke-linecap  ="round"
          stroke-linejoin ="round"
          inner_html=  move || glyph.get().svg()
        />
    }
}
