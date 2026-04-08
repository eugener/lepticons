# Phase 2: IconPicker Crate (lepticons-picker 0.1.0)

Goal: Ship an embeddable, searchable icon selector for forms, editors, and dashboards. Nothing like it exists in the Leptos ecosystem.

## Crate Setup

### Cargo.toml

```toml
[package]
name = "lepticons-picker"
version = "0.1.0"
edition = "2021"
description = "Embeddable icon picker component for Leptos, powered by Lucide icons."
license = "MIT"
repository = "https://github.com/eugener/lepticons"
homepage = "https://lepticons.vercel.app"
documentation = "https://docs.rs/lepticons-picker"
keywords = ["icons", "picker", "leptos", "component"]
categories = ["gui", "web-programming"]

[dependencies]
leptos = "0.8"
lepticons = { version = "0.9", path = "../lepticons" }
convert_case = { workspace = true }
web-sys = { version = "0.3", features = [
    "ScrollIntoViewOptions", "ScrollBehavior", "ScrollLogicalPosition",
] }
```

No `floating-ui-leptos` in v0.1 -- popover positioning is CSS-only.

### File Structure

```
lepticons-picker/
  Cargo.toml
  README.md
  src/
    lib.rs                     # Re-exports all public components
    icon_search.rs             # IconSearch component
    icon_grid.rs               # IconGrid + IconCell components
    category_filter.rs         # CategoryFilter component
    icon_picker.rs             # IconPicker (inline composed)
    icon_picker_popover.rs     # IconPickerPopover (popover wrapper)
```

## Components

### IconSearch

Debounced text input for filtering icons. Extracted from demo `demo/src/icons_view.rs:39-60`.

```rust
/// Debounced search input for filtering icons.
///
/// Emits the filter value after `debounce_ms` of inactivity (default 150ms).
/// Programmatic changes to `value` are emitted immediately (no debounce).
///
/// # Example
///
/// ```rust,ignore
/// let (filter, set_filter) = signal(String::new());
/// <IconSearch value=filter on_change=set_filter />
/// ```
#[component]
pub fn IconSearch(
    /// Current filter value (controlled).
    #[prop(into)] value: Signal<String>,
    /// Called with the new filter value after debounce.
    on_change: WriteSignal<String>,
    /// Debounce delay in milliseconds.
    #[prop(default = 150)] debounce_ms: u64,
    /// Placeholder text.
    #[prop(into, optional)] placeholder: Option<TextProp>,
    /// CSS class for the outer container div.
    #[prop(into, optional)] class: Option<TextProp>,
    /// Whether to show the clear (X) button.
    #[prop(default = true)] show_clear: bool,
) -> impl IntoView
```

**Internal behavior:**
- Maintains a raw input signal that updates on every keystroke
- Schedules a debounced emit: after `debounce_ms`, if raw value hasn't changed, calls `on_change`
- When `value` signal changes externally (category click, programmatic clear), syncs `on_change` immediately
- Renders: search icon (LucideGlyph::Search) + input + optional clear button (LucideGlyph::X)

**Default rendering:**
```html
<div class="{class}">
  <Icon glyph=Search />
  <input type="text" placeholder="{placeholder}" />
  <Icon glyph=X on:click=clear />  <!-- if show_clear -->
</div>
```

### IconGrid

Selectable grid of icon cells. Extracted from demo `IconTable` + `IconCell` (demo/src/icons_view.rs:130-175).

```rust
/// Grid of icon cells with selection and tooltip support.
///
/// Filters icons using `LucideGlyph::find()` and displays them in a flex-wrap grid.
/// Clicking an icon invokes `on_select`.
///
/// # Example
///
/// ```rust,ignore
/// let (selected, set_selected) = signal(None::<LucideGlyph>);
/// <IconGrid
///     filter=filter_signal
///     selected=selected
///     on_select=Callback::new(move |icon| set_selected.set(Some(icon)))
/// />
/// ```
#[component]
pub fn IconGrid(
    /// Filter string (searched against name, tags, categories via LucideGlyph::find).
    #[prop(into)] filter: Signal<String>,
    /// Currently selected icon (used for highlight).
    #[prop(into)] selected: Signal<Option<LucideGlyph>>,
    /// Called when an icon cell is clicked.
    on_select: Callback<LucideGlyph>,
    /// CSS class for the grid container div.
    #[prop(into, optional)] class: Option<TextProp>,
    /// CSS class for each unselected icon cell.
    #[prop(into, optional)] cell_class: Option<TextProp>,
    /// CSS class for the selected icon cell.
    #[prop(into, optional)] cell_selected_class: Option<TextProp>,
    /// Icon size in the grid cells (default: "24").
    #[prop(into, optional)] icon_size: Option<TextProp>,
    /// Whether to show icon name tooltips on hover.
    #[prop(default = true)] tooltips: bool,
) -> impl IntoView
```

**Internal behavior:**
- Computes `LucideGlyph::find(filter)` reactively
- Renders each result as an `IconCell` (internal sub-component)
- Selected cell gets `cell_selected_class` instead of `cell_class`
- Clicked cell: calls `on_select`, scrolls into view with smooth behavior
- Tooltip: absolute-positioned div with icon name, shown on hover via CSS `group-hover`

**Default rendering:**
```html
<div class="{class}" style="display:flex;flex-wrap:wrap;gap:0.5rem">
  <div class="{cell_class|cell_selected_class}" style="position:relative;...">
    <Icon glyph=icon size="{icon_size}" />
    <div class="tooltip">{icon.name()}</div>  <!-- if tooltips -->
  </div>
  ...
</div>
```

**No virtual scroll.** 700 lightweight SVG elements render fine. If performance becomes an issue, add pagination in a follow-up.

### CategoryFilter

Category sidebar/list with icon counts. Extracted from demo demo/src/icons_view.rs:79-91.

```rust
/// Displays all icon categories with their icon counts.
///
/// Clicking a category invokes `on_select` with the category name (Title Case).
///
/// # Example
///
/// ```rust,ignore
/// <CategoryFilter
///     on_select=Callback::new(move |cat: String| set_filter.set(cat))
/// />
/// ```
#[component]
pub fn CategoryFilter(
    /// Called with the category title when clicked.
    on_select: Callback<String>,
    /// Currently active category (used for highlight). Empty string = none.
    #[prop(into, optional)] active: Option<Signal<String>>,
    /// CSS class for the container div.
    #[prop(into, optional)] class: Option<TextProp>,
    /// CSS class for each category row.
    #[prop(into, optional)] item_class: Option<TextProp>,
    /// CSS class for the active category row.
    #[prop(into, optional)] item_active_class: Option<TextProp>,
) -> impl IntoView
```

**Internal behavior:**
- Iterates `LucideGlyph::all_categories()`
- Each row: category title (left-aligned) + count (right-aligned)
- Click sets filter to category name
- Optional `active` signal highlights the current category

**Default rendering:**
```html
<div class="{class}">
  <div class="{item_class}" on:click=...>
    <span>Accessibility</span>
    <span>15</span>
  </div>
  ...
</div>
```

### IconPicker (inline mode)

Composed from IconSearch + CategoryFilter + IconGrid.

```rust
/// Inline icon picker with search, category filter, and selectable grid.
///
/// Drop this into a form, settings panel, or editor to let users pick an icon.
///
/// # Example
///
/// ```rust,ignore
/// let (icon, set_icon) = signal(None::<LucideGlyph>);
/// <IconPicker
///     selected=icon
///     on_select=Callback::new(move |g| set_icon.set(Some(g)))
/// />
/// ```
#[component]
pub fn IconPicker(
    /// Currently selected icon.
    #[prop(into)] selected: Signal<Option<LucideGlyph>>,
    /// Called when an icon is selected.
    on_select: Callback<LucideGlyph>,
    /// Whether to show the category sidebar.
    #[prop(default = true)] show_categories: bool,
    /// Whether to show the search bar.
    #[prop(default = true)] show_search: bool,
    /// CSS class for the outer container.
    #[prop(into, optional)] class: Option<TextProp>,
    /// Maximum height (enables scroll). Default: "400px".
    #[prop(into, optional)] max_height: Option<TextProp>,
) -> impl IntoView
```

**Layout:**
```
+-------------------------------------------+
| [Search input........................]  X  |
+----------+--------------------------------+
| Category | [icon] [icon] [icon] [icon]     |
| List     | [icon] [icon] [icon] [icon]     |
| with     | [icon] [icon] [icon] [icon]     |
| counts   | ...                             |
+----------+--------------------------------+
```

**Internal state:**
- `filter: RwSignal<String>` -- raw search text
- `debounced_filter: RwSignal<String>` -- debounced version passed to IconGrid
- Category click sets filter to category name
- Search input overrides category filter
- Clear button resets filter

**Controlled state only.** `selected` and `on_select` are owned by the parent. No hidden internal selection state.

### IconPickerPopover (popover mode)

Wraps IconPicker in a positioned overlay triggered by a child element.

```rust
/// Popover that shows an IconPicker when a trigger element is clicked.
///
/// # Example
///
/// ```rust,ignore
/// let (icon, set_icon) = signal(None::<LucideGlyph>);
/// <IconPickerPopover
///     selected=icon
///     on_select=Callback::new(move |g| set_icon.set(Some(g)))
/// >
///     <button>"Choose icon"</button>
/// </IconPickerPopover>
/// ```
#[component]
pub fn IconPickerPopover(
    /// Currently selected icon.
    #[prop(into)] selected: Signal<Option<LucideGlyph>>,
    /// Called when an icon is selected.
    on_select: Callback<LucideGlyph>,
    /// Trigger element (the clickable thing that opens the popover).
    children: Children,
    /// CSS class for the popover panel.
    #[prop(into, optional)] class: Option<TextProp>,
    /// Whether selecting an icon closes the popover.
    #[prop(default = true)] close_on_select: bool,
    /// Popover width. Default: "480px".
    #[prop(into, optional)] width: Option<TextProp>,
    /// Popover height. Default: "400px".
    #[prop(into, optional)] height: Option<TextProp>,
) -> impl IntoView
```

**Internal state:**
- `open: RwSignal<bool>` -- popover visibility
- Click on trigger toggles open
- Click outside closes (document click listener, stop propagation on popover)
- Escape key closes
- If `close_on_select`, selecting an icon closes the popover

**Positioning:**
- CSS `position: relative` on wrapper, `position: absolute` on panel
- Opens below trigger by default: `top: 100%; left: 0`
- Shadow + border for visual separation
- No `floating-ui-leptos` in v0.1 -- add in v0.2 if edge cases arise (viewport overflow, scroll containers)

**Rendering:**
```html
<div style="position:relative;display:inline-block">
  <div on:click=toggle>{children()}</div>
  {open.then(|| view! {
    <div style="position:absolute;top:100%;left:0;z-index:50;width:{width};height:{height}"
         class="{class}">
      <IconPicker selected=selected on_select=wrapped_on_select />
    </div>
  })}
</div>
```

## Theming

### Strategy

All components accept `class` props for full CSS override. For users who want quick defaults without writing CSS, components use CSS custom properties with inline style fallbacks.

### CSS Custom Properties

```css
:root {
  --lp-bg: #ffffff;
  --lp-bg-hover: #f5f5f5;
  --lp-bg-selected: rgba(234, 88, 12, 0.1);
  --lp-text: #1a1a1a;
  --lp-text-muted: #666666;
  --lp-border: #e5e5e5;
  --lp-border-selected: rgba(234, 88, 12, 0.8);
  --lp-accent: #ea580c;
  --lp-radius: 0.5rem;
  --lp-cell-size: 40px;
  --lp-tooltip-bg: rgba(234, 88, 12, 0.9);
  --lp-tooltip-text: #ffffff;
}
```

### How components use them

Components set inline styles using `var(--lp-bg, #ffffff)` (with fallback). If user passes a `class` prop, they can override everything via CSS specificity. If they set CSS variables, the defaults change globally.

### Tailwind users

Pass Tailwind classes via `class`, `cell_class`, etc. props. The inline style fallbacks have lower specificity and won't conflict.

## Dependencies

| Dependency | Why | Weight |
|------------|-----|--------|
| `leptos` | Framework | Already in user's tree |
| `lepticons` | Icon data + Icon component | Already in user's tree |
| `convert_case` | Title Case for category names | Tiny, already in workspace |
| `web-sys` | ScrollIntoView, event handling | Already in user's tree (WASM apps) |

No additional dependencies. No JS. No CSS framework requirement.

## What This Crate Does NOT Include

- Icon detail/preview panel (demo-app-specific)
- Copy/export (SVG, PNG, data URL -- demo-app-specific)
- Theme toggle (app-level concern)
- Icon animations (separate crate: lepticons-animate)
- Virtual scroll (not needed for ~700 items)

## README Outline

```markdown
# lepticons-picker

Embeddable icon picker for Leptos applications, powered by Lucide icons.

## Quick Start
[IconPickerPopover example]

## Components
- IconPicker -- inline picker
- IconPickerPopover -- popover picker
- IconSearch -- standalone search input
- IconGrid -- standalone icon grid
- CategoryFilter -- standalone category list

## Customization
[class props + CSS variables]

## Requirements
- Leptos 0.8.x
- lepticons 0.9.x
```
