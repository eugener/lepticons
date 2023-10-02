extern crate core;

use std::collections::{BTreeMap, HashSet};

use base64::engine::general_purpose;
use base64::*;
use cached::proc_macro::cached;

use convert_case::Case::Title;
use convert_case::Casing;
use lucide_icon_data::LucideGlyph;
use strum::{EnumProperty, IntoEnumIterator};
use weezl::{decode::Decoder, BitOrder};

use crate::lucide_icon_data;


/**
 * Decompresses the svg string of a LucideGlyph
 * Uses caching to speed up the process for repeated calls
 * The speed up is significant, approx 8-9x
 */
#[cached]
fn decompress_str(input: String) -> String {
    let compressed = general_purpose::STANDARD_NO_PAD.decode(input).unwrap();
    let decompressed = Decoder::new(BitOrder::Msb, 9)
        .decode(&compressed.to_vec())
        .unwrap();
    return String::from_utf8(decompressed).unwrap();
}

/**
 * Cached function to convert a command delimited string to a vector of strings
 */
#[cached]
fn str_to_vec(input: String) -> Vec<String> {
    input.trim()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

pub trait Glyph: Clone {
    fn svg(&self) -> String;
}

impl Glyph for LucideGlyph {
    fn svg(&self) -> String {
        decompress_str(self.get_str("svg").expect("get svg").to_string())
    }
}

impl LucideGlyph {
    pub fn categories(&self) -> Vec<String> {
        str_to_vec(self.get_str("categories").unwrap_or("").to_string())
    }

    pub fn tags(&self) -> Vec<String> {
        str_to_vec(self.get_str("tags").unwrap_or("").to_string())
    }

    pub fn contributors(&self) -> Vec<String> {
        str_to_vec(self.get_str("contributors").unwrap_or("").to_string())
    }

    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn all_categories() -> BTreeMap<String, u16> {
        let mut categories: BTreeMap<String, u16> = BTreeMap::new();
        for icon in LucideGlyph::iter() {
            for category in icon.categories() {
                let count = categories
                    .entry(category.to_case(Title).to_string())
                    .or_insert(0);
                *count += 1;
            }
        }
        categories
    }

    fn search_base(&self) -> HashSet<String> {
        let mut acc = HashSet::from([self.name().to_lowercase()]);
        acc.extend(self.tags().iter().map(|tag| tag.to_string()));
        acc.extend(self.categories().iter().map(|cat| cat.to_string()));
        acc
    }

    pub fn find(filter: &str) -> Vec<LucideGlyph> {
        if filter.is_empty() {
            return LucideGlyph::iter().collect::<Vec<_>>();
        }

        LucideGlyph::iter()
            .filter(|icon| icon.search_base().iter().any(|tag| tag.contains(filter)))
            .collect::<Vec<_>>()
    }
}


// #[cfg(test)]
// mod tests {
//     use std::time::Instant;
//     use super::*;
//
//     #[test]
//     fn bench_svg_decompression() {
//        let start_time = Instant::now();
//        decompress_all();
//        let elapsed_time = start_time.elapsed();
//        println!("Elapsed time: {:?}", elapsed_time);
//     }
//
//     fn decompress_all() {
//         for _ in 1..10 {
//             for icon in LucideGlyph::iter() {
//                 icon.svg();
//             }
//         }
//     }
//
// }