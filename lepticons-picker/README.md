# lepticons-picker

Embeddable icon picker for [Leptos](https://leptos.dev) applications, powered by [Lucide](https://lucide.dev) icons.

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

Inline picker with search, category filter, and selectable grid.

```rust
<IconPicker
    selected=icon
    on_select=Callback::new(move |g| set_icon.set(Some(g)))
/>
```

### IconPickerPopover

Popover wrapper -- shows the picker in a dropdown when a trigger element is clicked.

### IconSearch

Standalone debounced search input with search/clear icons.

### IconGrid

Standalone selectable icon grid with tooltips.

### CategoryFilter

Standalone category list with icon counts.

## Customization

All components accept `class` props for full CSS override. Default styling uses CSS custom properties with inline fallbacks:

```css
:root {
  --lp-bg: #ffffff;
  --lp-bg-hover: #f5f5f5;
  --lp-bg-selected: rgba(192, 58, 23, 0.1);
  --lp-text: #1a1a1a;
  --lp-text-muted: #666666;
  --lp-border: #e5e5e5;
  --lp-border-selected: rgba(192, 58, 23, 0.8);
  --lp-radius: 0.5rem;
  --lp-cell-size: 40px;
  --lp-tooltip-bg: rgba(192, 58, 23, 0.9);
  --lp-tooltip-text: #ffffff;
}
```

Tailwind users: pass classes via `class`, `cell_class`, etc. props. Inline style fallbacks have lower specificity and won't conflict.

## Requirements

- Leptos 0.8.x
- lepticons 0.9.x

## License

MIT
