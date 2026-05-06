# lepticons-picker

Embeddable icon picker for [Leptos](https://leptos.dev) applications, powered by [Lucide](https://lucide.dev) icons.

Search, browse, and copy 1,700 icons with a drop-in component. Fully keyboard-driven, themable through CSS variables, and persists recent picks to `localStorage`.

Part of the [Lepticons](https://github.com/eugener/lepticons) toolkit.

## Quick Start

```rust
use lepticons::LucideGlyph;
use lepticons_picker::IconPickerPopover;
use leptos::prelude::*;

let (icon, set_icon) = signal(None::<LucideGlyph>);

view! {
    <IconPickerPopover
        selected=icon
        on_select=Callback::new(move |g| set_icon.set(Some(g)))
    >
        <button>"Choose icon"</button>
    </IconPickerPopover>
}
```

## Components

### IconPicker

Inline picker with search, category filter, MRU strip, copy-as-code dropdown, and selectable grid.

```rust
<IconPicker
    selected=icon
    on_select=Callback::new(move |g| set_icon.set(Some(g)))
/>
```

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `selected` | `Signal<Option<LucideGlyph>>` | required | Currently selected icon |
| `on_select` | `Callback<LucideGlyph>` | required | Fired when an icon is picked |
| `show_categories` | `bool` | `true` | Render the category sidebar |
| `show_search` | `bool` | `true` | Render the search bar |
| `mru_enabled` | `bool` | `true` | Persist selections + show "Recently used" strip |
| `mru_storage_key` | `&'static str` | `"lepticons-picker-mru"` | `localStorage` key (override for multi-instance) |
| `show_copy` | `bool` | `true` | Render the "Copy as" format dropdown + per-cell copy buttons |
| `class` | `TextProp` | -- | Outer container class |
| `max_height` | `TextProp` | `"400px"` | Container max-height (enables grid scroll) |

### IconPickerPopover

Popover wrapper -- shows the picker in a dropdown when the trigger is clicked.

```rust
<IconPickerPopover
    selected=icon
    on_select=Callback::new(move |g| set_icon.set(Some(g)))
    width="540px"
    height="440px"
>
    <button>"Choose icon"</button>
</IconPickerPopover>
```

Dismisses on Escape, outside-click (`mousedown` with target test so CSS resize handles inside the panel don't close it), or selection. The panel size is captured on every close and reused as the initial size on next open, so user resizes survive close/reopen cycles. Pass `class="resize-x overflow-hidden"` (Tailwind) to enable horizontal user-resize.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `selected` | `Signal<Option<LucideGlyph>>` | required | Currently selected icon |
| `on_select` | `Callback<LucideGlyph>` | required | Fired when an icon is picked |
| `class` | `TextProp` | -- | Class applied to the popover panel |
| `close_on_select` | `bool` | `true` | Auto-dismiss after a pick |
| `width` | `TextProp` | `"480px"` | Initial panel width |
| `height` | `TextProp` | `"400px"` | Initial panel height |
| `aria_label` | `TextProp` | `"Choose an icon"` | Dialog label |

### IconGrid

Standalone selectable icon grid with tooltips, keyboard navigation, and an optional copy-as-code button per cell.

```rust
<IconGrid
    filter=filter_signal
    selected=icon
    on_select=Callback::new(move |g| set_icon.set(Some(g)))
/>
```

The grid uses CSS Grid (`grid-template-columns: repeat(auto-fill, minmax(2.5rem, 1fr))`) by default, with a keyed `<For>` so cells survive filter changes without remounting. Cells are `role="gridcell"` with `aria-label` and `aria-selected`, and use a roving `tabindex` so the grid takes a single tab stop.

**Keyboard:** Arrow keys move focus; Home / End jump to first / last; PageUp / PageDown move five rows; Enter or Space selects.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `filter` | `Signal<String>` | required | Search string (forwarded to `LucideGlyph::find`) |
| `selected` | `Signal<Option<LucideGlyph>>` | required | Currently selected icon |
| `on_select` | `Callback<LucideGlyph>` | required | Fired when a cell is activated |
| `class` | `TextProp` | -- | Outer grid class (suppresses default grid style when set) |
| `cell_class` / `cell_selected_class` | `TextProp` | -- | Per-cell class overrides |
| `tooltip_class` | `TextProp` | -- | Tooltip class override |
| `tooltips` | `bool` | `true` | Show name tooltips on hover/focus |
| `icon_size` / `icon_stroke` / `icon_stroke_width` / `icon_fill` | `TextProp` | `"24"` / `"currentColor"` / `"1.5"` / `"none"` | Forwarded to each `<Icon>` |
| `copy_format` | `Signal<IconCopyFormat>` | -- | When supplied, each cell renders a hover/focus copy button that writes the icon to the clipboard in this format |

When `filter` returns zero hits, the grid renders an empty-state message with a link to the upstream Lucide issue tracker.

### MruStrip

Horizontal strip of recently-used icons, backed by `localStorage` (via `mru::load` / `mru::save` / `mru::push_into`).

```rust
use lepticons_picker::{mru, MruStrip};

let mru_signal = RwSignal::new(mru::load("my-mru"));
let on_select = Callback::new(move |g: LucideGlyph| { /* ... */ });

view! { <MruStrip mru=mru_signal on_select=on_select /> }
```

Themable via `class` / `header_class` / `item_class` / `header_text` / `show_header` / `icon_size` / `icon_stroke` / `icon_stroke_width` / `icon_fill` -- when the corresponding class prop is set, default inline styles are suppressed.

### IconSearch

Standalone debounced search input. Renders a trailing `<kbd>/</kbd>` shortcut hint when empty.

```rust
<IconSearch
    value=filter
    on_change=Callback::new(move |v| set_filter.set(v))
/>
```

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Signal<String>` | required | Controlled value |
| `on_change` | `Callback<String>` | required | Fired after debounce |
| `debounce_ms` | `u64` | `150` | Idle ms before emitting |
| `placeholder` / `aria_label` | `TextProp` | `"Search icons..."` / `"Search icons"` | Input attributes |
| `class` / `input_class` / `kbd_class` / `clear_class` | `TextProp` | -- | Element class overrides |
| `icon_size` / `icon_stroke` | `TextProp` | `"18"` / muted var | Search/clear icon styling |
| `show_clear` | `bool` | `true` | Render the X clear button |
| `shortcut_hint` | `bool` | `true` | Render the `/` kbd chip when empty |
| `input_ref` | `NodeRef<Input>` | -- | Lets parents focus the input (e.g. for a `/` shortcut) |

Note: `IconSearch` does not register a `/` listener itself -- consumers wire it at the right scope. The bundled `IconPicker` does this at the picker container; standalone users typically do it at the window.

### CategoryFilter

Standalone category list with icon counts. Click sets the search filter to the category name.

## IconCopyFormat

Format passed to `IconGrid`'s `copy_format` prop and used by `IconPicker`'s built-in dropdown.

```rust
pub enum IconCopyFormat {
    Variant,    // "LucideGlyph::Heart"
    Component,  // "<Icon glyph=LucideGlyph::Heart />"
    Svg,        // <svg ... viewBox="0 0 24 24" ...>...</svg>
}
```

`IconCopyFormat::ALL` returns a slice for iteration; `from_id` / `id` round-trip via stable string identifiers; `render(icon)` produces the formatted code.

## mru module (public)

```rust
use lepticons_picker::mru;

let list = mru::load("my-storage-key");                 // -> Vec<LucideGlyph>
mru::push_into(&mut list, LucideGlyph::Heart);          // dedup + cap at 8
mru::save("my-storage-key", &list);                     // best-effort persist
```

Stored as a JSON array of variant *names* (not discriminants), so the list survives Lucide releases that add or remove icons. Unknown names are pruned silently on load.

## Customization

All components accept `class` props for full CSS override. Default styling uses CSS custom properties with inline fallbacks:

```css
.lp-themed {
    --lp-bg: hsl(0 0% 98%);
    --lp-text: var(--my-foreground);
    --lp-text-muted: var(--my-muted);
    --lp-border: var(--my-border);
    --lp-radius: 0.5rem;
    --lp-bg-selected: hsl(14 75% 43% / 0.12);
    --lp-border-selected: hsl(14 75% 43%);
    --lp-tooltip-bg: hsl(14 75% 43%);
    --lp-tooltip-text: white;
    --lp-link: hsl(14 75% 43%);
    --lp-kbd-bg: hsl(0 0% 95%);
    --lp-copy-bg: hsl(0 0% 100%);
}
```

Tailwind users: pass classes via `class`, `cell_class`, `kbd_class`, etc. Inline style fallbacks have lower specificity and won't conflict.

## Related Crates

- [lepticons](https://crates.io/crates/lepticons) -- core icon library (required dependency)
- [lepticons-animate](https://crates.io/crates/lepticons-animate) -- stroke draw-in and CSS utility animations

## Requirements

- Leptos 0.8.x
- lepticons 0.12.x

## Demo

[lepticons.9bits.cc/components](https://lepticons.9bits.cc/components) -- inline picker, popover trigger, live theming, keyboard nav, copy-as-code, and MRU.

## License

MIT
