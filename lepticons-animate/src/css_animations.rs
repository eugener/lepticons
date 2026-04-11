use leptos::prelude::*;

const ANIMATION_CSS: &str = "\
@keyframes lepticons-spin {\
  from { transform: rotate(0deg); }\
  to { transform: rotate(360deg); }\
}\
@keyframes lepticons-pulse {\
  0%, 100% { opacity: 1; }\
  50% { opacity: 0.5; }\
}\
@keyframes lepticons-bounce {\
  0%, 100% { transform: translateY(0); }\
  50% { transform: translateY(-25%); }\
}\
@keyframes lepticons-ping {\
  75%, 100% { transform: scale(2); opacity: 0; }\
}\
.lepticons-spin { animation: lepticons-spin 1s linear infinite; }\
.lepticons-pulse { animation: lepticons-pulse 2s ease-in-out infinite; }\
.lepticons-bounce { animation: lepticons-bounce 1s ease infinite; }\
.lepticons-ping { animation: lepticons-ping 1s cubic-bezier(0, 0, 0.2, 1) infinite; }\
";

/// Injects the CSS animation utility classes into the document.
///
/// Place this component once at the top of your app to enable
/// `.lepticons-spin`, `.lepticons-pulse`, `.lepticons-bounce`, and `.lepticons-ping`
/// CSS classes on any `<Icon>` component.
///
/// # Example
///
/// ```rust,ignore
/// use lepticons_animate::AnimationStyles;
/// use lepticons::{Icon, LucideGlyph};
///
/// // In your app root:
/// view! {
///     <AnimationStyles />
///     <Icon glyph=LucideGlyph::Loader class="lepticons-spin" />
///     <Icon glyph=LucideGlyph::Bell class="lepticons-bounce" />
/// }
/// ```
#[component]
pub fn AnimationStyles() -> impl IntoView {
    view! {
        <style>{ANIMATION_CSS}</style>
    }
}
