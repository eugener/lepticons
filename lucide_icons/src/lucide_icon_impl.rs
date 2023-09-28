extern crate core;


use std::collections::{BTreeMap, HashSet};

use base64::*;
use base64::engine::general_purpose;
use convert_case::Casing;
use convert_case::Case::Title;
use lucide_icon_data::LucideGlyph;
use strum::{EnumProperty, IntoEnumIterator};
use weezl::{BitOrder, decode::Decoder};

use crate::lucide_icon_data;

fn decompress_str(input: &str) -> String {
    let compressed = general_purpose::STANDARD_NO_PAD.decode(input).unwrap();
    let decompressed = Decoder::new(BitOrder::Msb, 9)
        .decode(&compressed.to_vec())
        .unwrap();
    return String::from_utf8(decompressed).unwrap();
}

pub trait Glyph {
    fn svg(&self) -> String;
}

impl Glyph for LucideGlyph {
    fn svg(&self) -> String {
        decompress_str(self.get_str("svg").expect("get svg"))
    }
}

impl LucideGlyph {

    pub fn categories(&self) -> Vec<&str> {
        self.get_str("categories")
            .expect("get categories")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn tags(&self) -> Vec<&str> {
        self.get_str("tags")
            .expect("get tags")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn contributors(&self) -> Vec<&str> {
        self.get_str("contributors")
            .expect("get contributors")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn all_categories() -> BTreeMap<String, u16> {

        let mut categories: BTreeMap<String, u16> = BTreeMap::new();
        for icon in LucideGlyph::iter() {
            for category in icon.categories() {
                let count = categories.entry(category.to_case(Title).to_string()).or_insert(0);
                *count += 1;
            }
        }
        categories
    }

    fn search_base(&self) -> HashSet<String> {
        let mut acc = HashSet::from([self.name().to_lowercase()]);
        acc.extend(  self.tags().iter().map(|tag| tag.to_string()));
        acc.extend(  self.categories().iter().map(|cat| cat.to_string()));
        acc
    }

    pub fn find(filter: &str) -> Vec<LucideGlyph> {

        if filter.is_empty() {
            return LucideGlyph::iter().collect::<Vec<_>>();
        }

        LucideGlyph::iter().filter(|icon| {
            icon.search_base().iter().any(|tag| tag.contains(filter))
        }).collect::<Vec<_>>()

    }
}