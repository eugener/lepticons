mod lucide_icon_data;
mod lucide_icon_impl;

pub use lucide_icon_data::LucideGlyph;
pub use lucide_icon_impl::Glyph;

use leptos::MaybeSignal;
use leptos::*;
// use leptos::logging::log;

const DEFAULT_SIZE: u16 = 24;
const DEFAULT_FILL: &'static str = "none";
const DEFAULT_STROKE: &'static str = "currentColor";
const DEFAULT_STROKE_WIDTH: f32 = 1.5;

// TODO cache decompressed svg

#[component]
pub fn Icon<T: Glyph + 'static>(
    glyph: MaybeSignal<T>,
    #[prop(into, default = MaybeSignal::from(""))] class: MaybeSignal<&'static str>,
    #[prop(default = MaybeSignal::from(DEFAULT_SIZE))] size: MaybeSignal<u16>,
    #[prop(default = MaybeSignal::from(DEFAULT_FILL))] fill: MaybeSignal<&'static str>,
    #[prop(default = MaybeSignal::from(DEFAULT_STROKE))] stroke: MaybeSignal<&'static str>,
    #[prop(default = MaybeSignal::from(DEFAULT_STROKE_WIDTH))] stroke_width: MaybeSignal<f32>,
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
