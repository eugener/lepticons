//! Format definitions and clipboard helpers for copying icon code.

use lepticons::{Glyph, LucideGlyph};

/// Code format used by the picker's "copy as" feature.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum IconCopyFormat {
    /// Bare enum variant: `LucideGlyph::Heart`.
    #[default]
    Variant,
    /// Full Leptos component invocation: `<Icon glyph=LucideGlyph::Heart />`.
    Component,
    /// Raw SVG markup with the standard 24x24 viewport.
    Svg,
}

impl IconCopyFormat {
    /// All known formats, in display order.
    pub const ALL: &'static [Self] = &[Self::Variant, Self::Component, Self::Svg];

    /// Short user-facing label for menus and buttons.
    pub fn label(self) -> &'static str {
        match self {
            Self::Variant => "Variant",
            Self::Component => "Component",
            Self::Svg => "SVG",
        }
    }

    /// Stable identifier suitable for `<option value=...>` mapping.
    pub fn id(self) -> &'static str {
        match self {
            Self::Variant => "variant",
            Self::Component => "component",
            Self::Svg => "svg",
        }
    }

    /// Parses a format identifier produced by [`Self::id`].
    pub fn from_id(s: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|f| f.id() == s)
    }

    /// Renders `icon` as code in this format.
    pub fn render(self, icon: LucideGlyph) -> String {
        match self {
            Self::Variant => format!("LucideGlyph::{}", icon.name()),
            Self::Component => format!("<Icon glyph=LucideGlyph::{} />", icon.name()),
            Self::Svg => format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" \
                 viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" \
                 stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\">{}</svg>",
                icon.svg()
            ),
        }
    }
}

/// Best-effort clipboard write. No-op when there is no browser, no
/// clipboard API, or the browser denies permission. The browser's
/// promise is fired and forgotten.
pub fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        let clipboard = window.navigator().clipboard();
        let _ = clipboard.write_text(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_variant() {
        assert_eq!(
            IconCopyFormat::Variant.render(LucideGlyph::Heart),
            "LucideGlyph::Heart"
        );
    }

    #[test]
    fn renders_component() {
        assert_eq!(
            IconCopyFormat::Component.render(LucideGlyph::Heart),
            "<Icon glyph=LucideGlyph::Heart />"
        );
    }

    #[test]
    fn round_trips_id() {
        for fmt in IconCopyFormat::ALL.iter().copied() {
            assert_eq!(IconCopyFormat::from_id(fmt.id()), Some(fmt));
        }
        assert_eq!(IconCopyFormat::from_id("nope"), None);
    }
}
