# Phase 4: Animations (lepticons-animate 0.1.0)

Goal: CSS/SVG animation utilities for icon transitions. Lightweight crate, mostly CSS + a thin Leptos wrapper.

Status: Future -- ship after Phases 1-3 and gather feedback.

## Crate Setup

```toml
[package]
name = "lepticons-animate"
version = "0.1.0"
edition = "2021"
description = "Icon animations for Leptos -- stroke draw-in, morph, spin, pulse."
license = "MIT"

[dependencies]
leptos = "0.8"
lepticons = { version = "0.9", path = "../lepticons" }
web-sys = { version = "0.3", features = ["SvgElement", "SvgPathElement", "SvgGeometryElement"] }
```

## Animation Types

### Stroke Draw-In

Icon paths animate from invisible to fully drawn.

**Technique:** CSS `stroke-dasharray` + `stroke-dashoffset` + `@keyframes`.

**Component:**

```rust
/// Renders an icon with a stroke draw-in animation.
///
/// The icon's paths animate from empty to fully drawn over `duration_ms`.
///
/// # Example
///
/// ```rust,ignore
/// <DrawIcon glyph=LucideGlyph::Check duration_ms=500 />
/// ```
#[component]
pub fn DrawIcon(
    #[prop(into)] glyph: Signal<LucideGlyph>,
    /// Animation duration in milliseconds.
    #[prop(default = 600)] duration_ms: u32,
    /// Delay before animation starts in milliseconds.
    #[prop(default = 0)] delay_ms: u32,
    /// CSS easing function.
    #[prop(into, optional)] easing: Option<TextProp>,
    // ...standard Icon props (class, size, stroke, etc.)
) -> impl IntoView
```

**Implementation:**
1. Render the `<svg>` with all paths having `stroke-dasharray: {totalLength}; stroke-dashoffset: {totalLength}`
2. On mount, query each `<path>` element, call `getTotalLength()` via web-sys `SvgGeometryElement`
3. Set `stroke-dasharray` to total length
4. Animate `stroke-dashoffset` from total length to 0 via CSS transition or `@keyframes`

**Complexity:** Medium. Requires `web-sys` access to SVG DOM for path length calculation. Falls back to rendering without animation if `getTotalLength()` is unavailable.

### Utility CSS Animations

Ship as a CSS file that users optionally include. No Rust component needed -- just class names.

**File:** `lepticons-animate/animations.css`

```css
@keyframes lepticons-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes lepticons-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

@keyframes lepticons-bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-25%); }
}

@keyframes lepticons-ping {
  75%, 100% { transform: scale(2); opacity: 0; }
}

.lepticons-spin { animation: lepticons-spin 1s linear infinite; }
.lepticons-pulse { animation: lepticons-pulse 2s ease-in-out infinite; }
.lepticons-bounce { animation: lepticons-bounce 1s ease infinite; }
.lepticons-ping { animation: lepticons-ping 1s cubic-bezier(0, 0, 0.2, 1) infinite; }
```

**Usage:**

```rust
<Icon glyph=LucideGlyph::Loader class="lepticons-spin" />
<Icon glyph=LucideGlyph::Bell class="lepticons-bounce" />
```

### Icon Morph (stretch goal)

Transition between two SVG path sets (e.g., menu icon morphing into X icon).

**Technique:** Interpolate SVG `d` attribute values between source and target paths.

**Challenges:**
- Lucide icons have varying numbers of `<path>`, `<circle>`, `<line>` elements
- SVG path interpolation requires matching path segment counts
- May need a Rust SVG path parsing + interpolation library

**Assessment:** Complex. Defer to v0.2 or later. Evaluate if `svg-path-interpolator` or similar crates exist in the Rust ecosystem. Alternatively, use CSS `clip-path` transitions for a simpler visual effect.

## Integration with Core Library

Animations work with the standard `Icon` component via CSS classes. `DrawIcon` is the only component that requires a different rendering approach (it wraps `Icon` with additional DOM manipulation).

The animation crate depends on `lepticons` but `lepticons` does not depend on the animation crate. Users opt in by adding `lepticons-animate` to their dependencies.
