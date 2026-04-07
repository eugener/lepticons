use std::collections::BTreeMap;
use std::sync::OnceLock;

use convert_case::Case::Title;
use convert_case::Casing;
use lucide_icon_data::LucideGlyph;
use strum::{EnumProperty, IntoEnumIterator};

use crate::lucide_icon_data;

pub trait Glyph: Clone {
    fn svg(&self) -> String;
}

impl Glyph for LucideGlyph {
    fn svg(&self) -> String {
        self.get_str("svg").expect("get svg").to_string()
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
            // Single concatenated string: "name,tag1,tag2,cat1,cat2"
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
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn categories(&self) -> Vec<String> {
        self.get_str("categories")
            .unwrap_or("")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }

    pub fn tags(&self) -> Vec<String> {
        self.get_str("tags")
            .unwrap_or("")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }

    pub fn contributors(&self) -> Vec<String> {
        self.get_str("contributors")
            .unwrap_or("")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
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
            .map(|entry| entry.glyph.clone())
            .collect()
    }
}
