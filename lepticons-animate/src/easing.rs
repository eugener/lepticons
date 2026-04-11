use std::fmt;

/// CSS transition timing function for stroke draw-in animation.
///
/// # Example
///
/// ```rust,ignore
/// use lepticons_animate::{DrawIcon, Easing};
/// use lepticons::LucideGlyph;
///
/// <DrawIcon glyph=LucideGlyph::Check easing=Easing::EaseOut />
/// ```
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Easing {
    /// Constant speed (`linear`).
    Linear,
    /// Browser default curve (`ease`).
    Ease,
    /// Starts slow, accelerates (`ease-in`).
    EaseIn,
    /// Starts fast, decelerates (`ease-out`).
    EaseOut,
    /// Slow start and end, fast middle (`ease-in-out`).
    #[default]
    EaseInOut,
}

impl fmt::Display for Easing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_css())
    }
}

impl Easing {
    /// Returns the CSS `transition-timing-function` value.
    pub const fn as_css(&self) -> &'static str {
        match self {
            Easing::Linear => "linear",
            Easing::Ease => "ease",
            Easing::EaseIn => "ease-in",
            Easing::EaseOut => "ease-out",
            Easing::EaseInOut => "ease-in-out",
        }
    }
}
