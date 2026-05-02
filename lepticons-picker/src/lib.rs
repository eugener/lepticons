//! Embeddable icon picker components for Leptos, powered by Lucide icons.
//!
//! Provides searchable, filterable icon selection components that can be
//! dropped into forms, editors, and dashboards.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use lepticons_picker::IconPickerPopover;
//! use lepticons::LucideGlyph;
//! use leptos::prelude::*;
//!
//! let (icon, set_icon) = signal(None::<LucideGlyph>);
//! view! {
//!     <IconPickerPopover
//!         selected=icon
//!         on_select=Callback::new(move |g| set_icon.set(Some(g)))
//!     >
//!         <button>"Choose icon"</button>
//!     </IconPickerPopover>
//! }
//! ```

mod icon_search;
mod icon_grid;
mod category_filter;
mod icon_picker;
mod icon_picker_popover;
pub mod mru;
mod copy;

pub use icon_search::IconSearch;
pub use icon_grid::IconGrid;
pub use category_filter::CategoryFilter;
pub use icon_picker::IconPicker;
pub use icon_picker_popover::IconPickerPopover;
pub use copy::IconCopyFormat;

/// Version of the `lepticons-picker` crate, taken from `Cargo.toml` at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
