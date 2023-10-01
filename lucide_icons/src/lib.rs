mod lucide_icon_data;
mod lucide_icon_impl;

pub use lucide_icon_data::LucideGlyph;
pub use lucide_icon_impl::Glyph;

use leptos::RwSignal;
use leptos::*;
// use leptos::logging::log;

const DEFAULT_SIZE: &'static str = "24";
const DEFAULT_FILL: &'static str = "none";
const DEFAULT_STROKE: &'static str = "currentColor";
const DEFAULT_STROKE_WIDTH: &'static str = "1.5";

// TODO cache decompressed svg

#[component]
pub fn Icon<T: Glyph + 'static>(
    #[prop(into)] glyph: Signal<T>,
    #[prop(into, default = RwSignal::from(""))] class: RwSignal<&'static str>,
    #[prop(into, default = RwSignal::from(DEFAULT_SIZE))] size: RwSignal<&'static str>,
    #[prop(into, default = RwSignal::from(DEFAULT_FILL))] fill: RwSignal<&'static str>,
    #[prop(into, default = RwSignal::from(DEFAULT_STROKE))] stroke: RwSignal<&'static str>,
    #[prop(into, default = RwSignal::from(DEFAULT_STROKE_WIDTH))] stroke_width: RwSignal<
        &'static str,
    >,
) -> impl IntoView {
    // log!("Icon: {}", icon.svg());

    // let svg = create_memo(move |_| glyph.get().svg());
    // let glyph = glyph.get();
    let fSize = move || size.get();

    view! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class   = move || class.get()
          width   = fSize
          height  = fSize
          viewBox = move || format!("0 0 {} {}", size.get(), size.get())
          fill    = move || fill.get()
          stroke  = move || stroke.get()
          stroke-width    = move || stroke_width.get()
          stroke-linecap  ="round"
          stroke-linejoin ="round"
          inner_html      =  move || glyph.get().svg()
        />
    }
}
