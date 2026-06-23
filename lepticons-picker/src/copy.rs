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
    /// React / JSX / Svelte PascalCase tag: `<Heart />`.
    Jsx,
    /// Svelte PascalCase tag (byte-identical to [`Self::Jsx`]); exposed
    /// separately so users searching by framework name find a match.
    Svelte,
    /// Vue kebab-case tag: `<heart />`.
    Vue,
    /// Angular component: `<lucide-angular name="heart" />`.
    Angular,
    /// `data:image/svg+xml,...` URL ready for `src=` / `background-image:`.
    /// The inner SVG is percent-encoded per RFC 3986.
    DataUrl,
}

impl IconCopyFormat {
    /// All known formats, in display order.
    pub const ALL: &'static [Self] = &[
        Self::Variant,
        Self::Component,
        Self::Svg,
        Self::Jsx,
        Self::Svelte,
        Self::Vue,
        Self::Angular,
        Self::DataUrl,
    ];

    /// Short user-facing label for menus and buttons.
    pub fn label(self) -> &'static str {
        match self {
            Self::Variant => "Variant",
            Self::Component => "Component",
            Self::Svg => "SVG",
            Self::Jsx => "JSX",
            Self::Svelte => "Svelte",
            Self::Vue => "Vue",
            Self::Angular => "Angular",
            Self::DataUrl => "Data URL",
        }
    }

    /// Stable identifier suitable for `<option value=...>` mapping.
    pub fn id(self) -> &'static str {
        match self {
            Self::Variant => "variant",
            Self::Component => "component",
            Self::Svg => "svg",
            Self::Jsx => "jsx",
            Self::Svelte => "svelte",
            Self::Vue => "vue",
            Self::Angular => "angular",
            Self::DataUrl => "data-url",
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
            Self::Jsx | Self::Svelte => format!("<{} />", icon.name()),
            Self::Vue => format!("<{} />", icon.kebab_name()),
            Self::Angular => format!("<lucide-angular name=\"{}\" />", icon.kebab_name()),
            Self::DataUrl => format!(
                "data:image/svg+xml,{}",
                percent_encode(&Self::Svg.render(icon))
            ),
        }
    }
}

/// Percent-encodes `s` per RFC 3986, escaping everything outside the
/// unreserved set (`A-Z a-z 0-9 - _ . ~`). Suitable for embedding SVG
/// markup in a `data:image/svg+xml,...` URL.
fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push(hex_nibble(b >> 4));
                out.push(hex_nibble(b & 0x0F));
            }
        }
    }
    out
}

fn hex_nibble(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        _ => (b'A' + n - 10) as char,
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

    #[test]
    fn renders_framework_formats() {
        assert_eq!(
            IconCopyFormat::Jsx.render(LucideGlyph::ArrowRight),
            "<ArrowRight />"
        );
        assert_eq!(
            IconCopyFormat::Svelte.render(LucideGlyph::ArrowRight),
            "<ArrowRight />"
        );
        assert_eq!(
            IconCopyFormat::Vue.render(LucideGlyph::ArrowRight),
            "<arrow-right />"
        );
        assert_eq!(
            IconCopyFormat::Angular.render(LucideGlyph::ArrowRight),
            "<lucide-angular name=\"arrow-right\" />"
        );
    }

    #[test]
    fn renders_data_url() {
        let url = IconCopyFormat::DataUrl.render(LucideGlyph::Heart);
        assert!(url.starts_with("data:image/svg+xml,"));
        // Encoded SVG must contain percent-escapes for `<` and `"`.
        assert!(url.contains("%3C"), "url missing percent-encoded '<': {url}");
        assert!(url.contains("%22"), "url missing percent-encoded '\"': {url}");
        // Unreserved chars (letters, digits, ~ . _ -) pass through verbatim.
        assert!(url.contains("svg"));
    }

    #[test]
    fn percent_encode_unreserved_passthrough() {
        assert_eq!(percent_encode("abcXYZ012-_.~"), "abcXYZ012-_.~");
    }

    #[test]
    fn percent_encode_escapes_reserved() {
        assert_eq!(percent_encode("<>\""), "%3C%3E%22");
        assert_eq!(percent_encode(" "), "%20");
    }
}
