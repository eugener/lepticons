//! Tiny helpers for the picker's "themable on opt-in" pattern.
//!
//! Every visible element in the picker accepts an optional `class` /
//! `*_class` `TextProp`. When the caller supplies one, they own the look
//! entirely and the corresponding default inline style must be suppressed
//! so cascade order doesn't fight the caller's classes.
//!
//! These helpers collapse the resulting two-closure boilerplate at each
//! call site:
//!
//! ```ignore
//! class=move || theme::class_str(&class)
//! style=move || theme::style_str(&class, DEFAULT_STYLE)
//! ```
//!
//! `style_str` returns `""` when a class was supplied, exactly matching
//! the suppress-on-opt-in semantics used everywhere else in the crate.

use leptos::text_prop::TextProp;

/// Renders the optional class to its current string value, or `""` when
/// no class was supplied.
pub(crate) fn class_str(opt: &Option<TextProp>) -> String {
    opt.as_ref()
        .map(|c| c.get().to_string())
        .unwrap_or_default()
}

/// Returns `default_style` when no class was supplied, otherwise `""`
/// (so the caller's class wins without the default inline style fighting
/// it).
pub(crate) fn style_str(opt: &Option<TextProp>, default_style: &'static str) -> &'static str {
    if opt.is_some() {
        ""
    } else {
        default_style
    }
}
