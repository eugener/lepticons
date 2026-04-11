use lepticons::{Glyph, LucideGlyph};
use strum::IntoEnumIterator;

#[test]
fn find_returns_results() {
    let results = LucideGlyph::find("arrow");
    assert!(!results.is_empty());
}

#[test]
fn find_empty_returns_all() {
    let all = LucideGlyph::find("");
    assert_eq!(all.len(), LucideGlyph::count());
}

#[test]
fn find_case_insensitive() {
    let lower = LucideGlyph::find("arrow");
    let upper = LucideGlyph::find("Arrow");
    let mixed = LucideGlyph::find("ARROW");
    assert_eq!(lower.len(), upper.len());
    assert_eq!(lower.len(), mixed.len());
}

#[test]
fn find_multi_word_is_conjunctive() {
    let arrow = LucideGlyph::find("arrow");
    let arrow_right = LucideGlyph::find("arrow right");
    assert!(!arrow_right.is_empty());
    assert!(arrow_right.len() < arrow.len());
}

#[test]
fn by_name_known_icon() {
    assert!(LucideGlyph::by_name("Activity").is_some());
}

#[test]
fn by_name_case_sensitive() {
    assert!(LucideGlyph::by_name("activity").is_none());
}

#[test]
fn by_name_unknown_returns_none() {
    assert!(LucideGlyph::by_name("NotAnIcon").is_none());
}

#[test]
fn categories_not_empty() {
    let cats = LucideGlyph::all_categories();
    assert!(cats.len() > 30);
}

#[test]
fn icon_metadata_accessible() {
    let icon = LucideGlyph::by_name("Activity").unwrap();
    assert!(!icon.svg().is_empty());
    assert!(icon.tags().count() > 0);
    assert!(icon.categories().count() > 0);
}

#[test]
fn count_matches_iter() {
    assert_eq!(LucideGlyph::count(), LucideGlyph::iter().count());
}

#[test]
fn count_is_positive() {
    assert!(LucideGlyph::count() > 600);
}

#[test]
fn glyph_trait_returns_svg() {
    let icon = LucideGlyph::by_name("Search").unwrap();
    let svg = icon.svg();
    assert!(svg.contains("<"));
    assert!(svg.contains("path"));
}
