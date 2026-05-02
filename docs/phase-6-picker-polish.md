# Phase 6: Picker Polish -- PLANNED

Target: `lepticons-picker 0.3.0`. Driven by post-launch feedback. ~1 weekend of work.

## Goals

Convert the picker from "browse and click" to "fast keyboard-driven icon selector" without breaking the existing `IconPicker` and `IconPickerPopover` APIs.

## 6a. Keyboard Navigation

Today the grid is mouse-only. Add:

- Arrow keys (up/down/left/right) move selection within the grid
- Home/End jump to first/last visible icon
- PageUp/PageDown scroll one viewport
- Enter triggers `on_select`
- Escape closes the popover (popover variant only)
- `/` (forward slash) focuses the search input from anywhere in the picker
- Focus visible with a clear outline -- respects `:focus-visible`, not `:focus`

**Implementation notes:**

- Track `focused_index: RwSignal<Option<usize>>` separate from `selected: Signal<Option<LucideGlyph>>`
- Use `web-sys` `KeyboardEvent` on a wrapper `<div tabindex=0>`; don't bind global listeners
- `scrollIntoView({block: "nearest"})` on focus change so arrow keys scroll the grid
- Filter results recompute resets focused_index to 0

**Breaking?** No -- adds behavior, no signature changes.

## 6b. Recently Used (MRU)

Top row of the grid shows up to 8 recently selected icons. Click or arrow into them like any other cell.

**Storage:** `localStorage` key `lepticons-picker-mru`, JSON array of glyph names (strings, not enum -- enum variants may shift across versions). Look up at render time via `LucideGlyph::by_name`.

**Component prop:** `mru_enabled: bool` (default true) so users can opt out. `mru_storage_key: &'static str` (default `"lepticons-picker-mru"`) for apps that want isolated MRUs per picker instance.

**Edge cases:**

- SSR: `localStorage` doesn't exist server-side; gate with `cfg(target_arch = "wasm32")` or a runtime check
- User picks an icon that's no longer in the current Lucide release: skip silently, prune from MRU
- MRU section heading "Recently used" hidden when list is empty

## 6c. Copy as Code

Right-click context menu (or hover-action button) on a grid cell copies one of:

- `LucideGlyph::Heart` (default)
- `<Icon glyph=LucideGlyph::Heart />` (full component)
- Raw SVG markup

User chooses format via dropdown next to search.

The demo already has copy-to-clipboard for URLs -- extract to a shared `Clipboard::copy` helper in the picker crate (or a new `lepticons-internal` util crate -- prefer inlining for now).

## 6d. Empty State

When search has zero hits, replace the empty grid with:

> No icons match `"foobar"`. Search uses name, tags, and category.
> Missing an icon? [Request it on lucide-icons/lucide](https://github.com/lucide-icons/lucide/issues/new?template=icon_request.yml)

Subtle, helpful, drives requests upstream rather than to lepticons.

## 6e. Accessibility Pass

While in the picker code:

- Grid uses `role="grid"` with `role="row"` / `role="gridcell"`
- Each cell has `aria-label` from icon name
- Selected state via `aria-selected="true"`
- Search input has explicit `<label>` (currently icon-only)
- Picker container `role="dialog"` + `aria-label="Choose an icon"` in popover variant
- Test with VoiceOver / NVDA before shipping

## Integration

- All changes are in `lepticons-picker`. No changes to `lepticons` core or `lepticons-animate`.
- New props default to existing behavior -- no migration needed for v0.2.x users.
- Bump to `0.3.0` despite no breakage, because keyboard nav is a meaningful UX shift worth signalling.

## Acceptance

- Keyboard nav passes manual checklist (arrow/home/end/enter/escape/slash all work)
- MRU persists across page reload, prunes invalid entries
- Copy-as-code works in Chrome, Firefox, Safari (Clipboard API)
- Empty state renders with link
- `aria-*` audit passes axe-core spot check on demo
