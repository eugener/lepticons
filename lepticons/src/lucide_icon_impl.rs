use std::collections::BTreeMap;
use std::sync::OnceLock;

use convert_case::Case::Title;
use convert_case::Casing;
use lucide_icon_data::LucideGlyph;
use strum::{EnumProperty, IntoEnumIterator};

use crate::lucide_icon_data;

/// Trait for types that provide SVG content for rendering.
pub trait Glyph: Copy {
    /// Returns the inner SVG content as a static string.
    fn svg(&self) -> &'static str;
}

impl Glyph for LucideGlyph {
    fn svg(&self) -> &'static str {
        self.get_str("svg").unwrap_or("")
    }
}

/// Pre-built search entry: (icon variant, concatenated searchable text).
struct SearchEntry {
    glyph: LucideGlyph,
    text: String,
}

/// Global flat search index, built once on first use.
static SEARCH_INDEX: OnceLock<Vec<SearchEntry>> = OnceLock::new();

/// Global categories cache, built once on first use.
static CATEGORIES: OnceLock<BTreeMap<String, u16>> = OnceLock::new();

fn build_search_index() -> Vec<SearchEntry> {
    LucideGlyph::iter()
        .map(|glyph| {
            let name = format!("{:?}", glyph).to_case(convert_case::Case::Lower);
            let tags = glyph.get_str("tags").unwrap_or("");
            let categories = glyph.get_str("categories").unwrap_or("");
            let text = format!("{},{},{}", name, tags, categories);
            SearchEntry { glyph, text }
        })
        .collect()
}

fn build_categories() -> BTreeMap<String, u16> {
    let mut categories: BTreeMap<String, u16> = BTreeMap::new();
    for icon in LucideGlyph::iter() {
        let cats = icon.get_str("categories").unwrap_or("");
        for cat in cats.split(',') {
            let cat = cat.trim();
            if !cat.is_empty() {
                let count = categories
                    .entry(cat.to_case(Title).to_string())
                    .or_insert(0);
                *count += 1;
            }
        }
    }
    categories
}

impl LucideGlyph {
    /// Returns the variant name (e.g. "AArrowDown").
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    /// Returns the raw categories string from the icon metadata.
    pub fn categories_str(&self) -> &'static str {
        self.get_str("categories").unwrap_or("")
    }

    /// Returns the raw tags string from the icon metadata.
    pub fn tags_str(&self) -> &'static str {
        self.get_str("tags").unwrap_or("")
    }

    /// Returns categories as an iterator of string slices.
    pub fn categories(&self) -> impl Iterator<Item = &'static str> {
        self.categories_str()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
    }

    /// Returns tags as an iterator of string slices.
    pub fn tags(&self) -> impl Iterator<Item = &'static str> {
        self.tags_str()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
    }

    /// Returns contributors as an iterator of string slices.
    pub fn contributors(&self) -> impl Iterator<Item = &'static str> {
        self.get_str("contributors")
            .unwrap_or("")
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
    }

    /// Returns a sorted map of all categories and their icon count.
    /// Computed once and cached.
    pub fn all_categories() -> &'static BTreeMap<String, u16> {
        CATEGORIES.get_or_init(build_categories)
    }

    /// Finds icons matching the filter string.
    /// Uses a pre-built flat index for zero per-call allocations.
    pub fn find(filter: &str) -> Vec<LucideGlyph> {
        if filter.is_empty() {
            return LucideGlyph::iter().collect();
        }

        let index = SEARCH_INDEX.get_or_init(build_search_index);
        let terms: Vec<&str> = filter.split_whitespace().collect();

        index
            .iter()
            .filter(|entry| {
                terms
                    .iter()
                    .any(|term| entry.text.contains(term))
            })
            .map(|entry| entry.glyph)
            .collect()
    }
}
