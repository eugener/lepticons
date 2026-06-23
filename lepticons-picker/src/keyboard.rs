//! Small DOM-event helpers shared across the picker components.

use leptos::wasm_bindgen::JsCast;

/// Keyboard shortcuts the picker handles natively, as `(keys, description)`
/// pairs. Use this to render an in-app help dialog that stays in sync with
/// the actual binding set (rather than hand-copying the list).
///
/// Does **not** include host-app concerns such as opening / closing a
/// detail drawer or toggling a help dialog -- those are the caller's
/// responsibility.
pub const SHORTCUTS: &[(&str, &str)] = &[
    ("/", "Focus search"),
    ("Arrow keys", "Move focus across the grid"),
    ("Home / End", "Jump to first / last icon"),
    ("Page Up / Down", "Move five rows at a time"),
    ("Enter / Space", "Select the focused icon"),
];

/// Returns `true` when the keyboard event originated inside a text-input
/// element, i.e. the user is typing into an `<input>` or `<textarea>`.
///
/// Use this to short-circuit page-wide keyboard shortcuts (`/`, `?`, etc.)
/// so they don't steal keystrokes while the user is editing text.
///
/// Returns `false` when the event has no target, the target isn't an
/// `HtmlElement`, or the target's tag is anything other than `input` /
/// `textarea`. `contenteditable` regions are *not* considered typing
/// targets -- callers that need that should add their own check.
pub fn is_typing_target(ev: &web_sys::KeyboardEvent) -> bool {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        .map(|el| {
            let tag = el.tag_name();
            tag.eq_ignore_ascii_case("input") || tag.eq_ignore_ascii_case("textarea")
        })
        .unwrap_or(false)
}
