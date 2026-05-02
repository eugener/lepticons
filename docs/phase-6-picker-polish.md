# Phase 6: Picker Polish -- COMPLETE

Shipped as `lepticons-picker 0.3.0`. Driven by post-launch feedback.

## What landed

- **6a Keyboard nav** -- arrow / Home / End / PageUp / PageDown / Enter / Space / Esc / `/`. Roving `tabindex`, focus tracked by glyph (not index) so cells survive filter changes via the keyed `<For>` reconciliation.
- **6b MRU** -- `pub mod mru` with `load` / `save` / `push_into`. Default `localStorage` key `"lepticons-picker-mru"`, capped at 8, JSON of glyph variant names so the list survives icon additions/removals across Lucide releases. Unknown names are pruned silently. Surfaced via the new public `MruStrip` component, which `IconPicker` also consumes internally.
- **6c Copy as code** -- new public `IconCopyFormat` enum (`Variant` / `Component` / `Svg`) with `render(icon) -> String` and stable `id` round-trip. `IconGrid` accepts an optional `copy_format: Signal<IconCopyFormat>` that turns on a hover/focus copy button per cell. `IconPicker` wires an internal format signal to a `<select>` next to the search bar.
- **6d Empty state** -- `IconGrid` renders a `role=status` block with a link to the upstream Lucide issue tracker when the filter has zero hits.
- **6e A11y** -- `role=grid` + `role=gridcell` + `aria-label` (kebab name) + `aria-selected` on cells; popover panel `role=dialog` + `aria-modal=true` + configurable `aria_label` (default "Choose an icon"); search input `role=searchbox` + `aria-label`.

## What also got rebuilt

- **CSS-grid layout** by default (`grid-template-columns: repeat(auto-fill, minmax(2.5rem, 1fr))`); demo overrides via class still work because inline grid style is suppressed when `class` is set.
- **Themable `IconSearch`** -- `class` / `input_class` / `kbd_class` / `clear_class` / `icon_size` / `icon_stroke` / `shortcut_hint` / `input_ref` props. Hides the native `::-webkit-search-cancel-button` so it doesn't double up with the X clear button.
- **`IconPickerPopover`** -- outside-close switched from `click` to `mousedown` with a target test against panel/trigger refs, so a CSS resize drag whose mouseup lands outside the panel no longer closes the popover. Panel size is captured on every close (offset_width/offset_height) and reused as the initial size on next open.
- **`<For>` keyed reconciliation** in `IconGrid` so cells aren't remounted across filter changes.

## Not in scope (deferred)

- Virtualization of `IconGrid` for the empty-filter case.
- Mobile drawer for the demo's icons-page sidebar.

---

Original plan below for reference.

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
