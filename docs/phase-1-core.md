# Phase 1: Core Library Improvements (lepticons 0.9.0)

Goal: Make lepticons production-ready with reactive props, name-based lookup, SSR verification, and tests.

Breaking changes: None. All existing usage patterns continue to work.

## 1a. Reactive Props on `Icon` Component

Current props are `&'static str` -- not reactive, can't accept `String` or signals.

### Current API (lib.rs)

```rust
#[component]
pub fn Icon(
    #[prop(into)] glyph: Signal<LucideGlyph>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = DEFAULT_SIZE)] size: &'static str,
    #[prop(default = DEFAULT_FILL)] fill: &'static str,
    #[prop(default = DEFAULT_STROKE)] stroke: &'static str,
    #[prop(default = DEFAULT_STROKE_WIDTH)] stroke_width: &'static str,
) -> impl IntoView
```

### New API

```rust
use leptos::text_prop::TextProp;

#[component]
pub fn Icon(
    #[prop(into)] glyph: Signal<LucideGlyph>,
    #[prop(into, optional)] class: Option<TextProp>,
    #[prop(into, optional)] size: Option<TextProp>,
    #[prop(into, optional)] fill: Option<TextProp>,
    #[prop(into, optional)] stroke: Option<TextProp>,
    #[prop(into, optional)] stroke_width: Option<TextProp>,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "".into());
    let size = size.unwrap_or_else(|| DEFAULT_SIZE.into());
    let fill = fill.unwrap_or_else(|| DEFAULT_FILL.into());
    let stroke = stroke.unwrap_or_else(|| DEFAULT_STROKE.into());
    let stroke_width = stroke_width.unwrap_or_else(|| DEFAULT_STROKE_WIDTH.into());

    view! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class=move || class.get()
          width=move || size.get()
          height=move || size.get()
          viewBox="0 0 24 24"
          fill=move || fill.get()
          stroke=move || stroke.get()
          stroke-width=move || stroke_width.get()
          stroke-linecap="round"
          stroke-linejoin="round"
          inner_html=move || glyph.get().svg()
        />
    }
}
```

### Why Option<TextProp> instead of TextProp with default

`TextProp` has no const constructor, so `#[prop(default = TextProp::from("24"))]` doesn't compile.
`Option<TextProp>` with `#[prop(into, optional)]` defaults to `None`, then we unwrap with fallback.

### Backwards compatibility

`TextProp` implements `From<&str>`, `From<String>`, `From<Oco<str>>`, and `From<Fn() -> String>`.
With `#[prop(into)]`, existing code like `<Icon glyph=LucideGlyph::Search size="32" />` compiles unchanged.

### What this enables

```rust
// Static (same as before)
<Icon glyph=LucideGlyph::Search size="32" />

// Reactive size from signal
let (size, set_size) = signal("24".to_string());
<Icon glyph=LucideGlyph::Search size=move || size.get() />

// Reactive class
<Icon glyph=LucideGlyph::Heart class=move || if active.get() { "text-red-500" } else { "text-gray-400" } />
```

## 1b. Add `EnumString` Derive for Name-Based Lookup

### Code Generator Change

File: `codegen/src/generate_lucide.rs`

Change the derive line (around line 54-58):

```rust
// Before
"#[derive(EnumIter, EnumProperty, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]"

// After
"#[derive(EnumIter, EnumProperty, EnumString, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]"
```

Change the import line:

```rust
// Before
use strum_macros::{EnumIter, EnumProperty};

// After
use strum_macros::{EnumIter, EnumProperty, EnumString};
```

### New Methods in lucide_icon_impl.rs

```rust
use std::str::FromStr;

static COUNT: OnceLock<usize> = OnceLock::new();

impl LucideGlyph {
    /// Looks up an icon by its variant name (e.g. "Activity", "ArrowRight").
    /// Returns None if the name doesn't match or the icon's category feature is disabled.
    pub fn by_name(name: &str) -> Option<LucideGlyph> {
        LucideGlyph::from_str(name).ok()
    }

    /// Returns the total number of available icon variants.
    pub fn count() -> usize {
        *COUNT.get_or_init(|| LucideGlyph::iter().count())
    }
}
```

### Why by_name() matters

Enables dynamic icon resolution from config, database, URL params:

```rust
// From URL parameter
let icon_name = params.get("icon"); // "Activity"
if let Some(glyph) = LucideGlyph::by_name(icon_name) {
    view! { <Icon glyph=glyph /> }
}
```

Note: `from_str` only matches variants whose category feature is enabled. If "Activity" requires `feature = "medical"` and that feature is off, `by_name("Activity")` returns `None`.

## 1c. SSR Verification

### What works already

- `Icon` component uses `inner_html` -- Leptos renders this as HTML string in SSR mode
- `OnceLock`-based search index: initialized once per process, thread-safe, works in SSR
- No `web-sys` dependency in the core crate -- no browser-only APIs

### What to verify

- Build `lepticons` with `--features ssr` (via leptos) and confirm compilation
- Render an `Icon` component in SSR mode and confirm correct HTML output
- Document that no additional feature flags are needed

### README addition

```markdown
## Rendering Modes

Lepticons works with all Leptos rendering modes (CSR, SSR, hydration).
No additional feature flags needed -- the library is rendering-mode agnostic.
```

## 1d. Tests

New file: `lepticons/tests/api_test.rs`

```rust
use lepticons::{Glyph, LucideGlyph};
use strum::IntoEnumIterator;

#[test]
fn find_returns_results() {
    let results = LucideGlyph::find("arrow");
    assert!(!results.is_empty());
}

#[test]
fn find_empty_returns_all() {
    let all = LucideGlyph::find("");
    assert_eq!(all.len(), LucideGlyph::count());
}

#[test]
fn find_multi_word() {
    let results = LucideGlyph::find("arrow right");
    assert!(!results.is_empty());
}

#[test]
fn by_name_known_icon() {
    assert!(LucideGlyph::by_name("Activity").is_some());
}

#[test]
fn by_name_case_sensitive() {
    assert!(LucideGlyph::by_name("activity").is_none()); // PascalCase only
}

#[test]
fn by_name_unknown_returns_none() {
    assert!(LucideGlyph::by_name("NotAnIcon").is_none());
}

#[test]
fn categories_not_empty() {
    let cats = LucideGlyph::all_categories();
    assert!(cats.len() > 30);
}

#[test]
fn icon_metadata_accessible() {
    if let Some(icon) = LucideGlyph::by_name("Activity") {
        assert!(!icon.svg().is_empty());
        assert!(icon.tags().count() > 0);
        assert!(icon.categories().count() > 0);
    }
}

#[test]
fn count_matches_iter() {
    assert_eq!(LucideGlyph::count(), LucideGlyph::iter().count());
}

#[test]
fn count_is_positive() {
    assert!(LucideGlyph::count() > 600);
}

#[test]
fn glyph_trait_returns_svg() {
    if let Some(icon) = LucideGlyph::by_name("Search") {
        let svg = icon.svg();
        assert!(svg.contains("<"));
        assert!(svg.contains("path"));
    }
}
```

## 1e. File Changes Summary

| File | Change |
|------|--------|
| `lepticons/src/lib.rs` | Reactive props via `Option<TextProp>` |
| `lepticons/src/lucide_icon_impl.rs` | Add `by_name()`, `count()`, `COUNT` static |
| `lepticons/src/lucide_icon_data.rs` | Regenerate with `EnumString` derive |
| `codegen/src/generate_lucide.rs` | Add `EnumString` to derives + import |
| `lepticons/Cargo.toml` | Version bump to 0.9.0 |
| `lepticons/README.md` | Document reactive props, by_name, SSR, count |
| `lepticons/tests/api_test.rs` | New: unit tests |

## 1f. Demo App Updates

After core changes, update demo to use new APIs where beneficial:

- Replace `ICON_COUNT` OnceLock in `demo/src/icons_view.rs` with `LucideGlyph::count()`
- No other demo changes required in Phase 1 (Phase 3 does the full refactor)
