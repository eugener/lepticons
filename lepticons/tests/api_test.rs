use lepticons::{Glyph, LucideGlyph};
use strum::IntoEnumIterator;

// --- find() ---

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
fn find_no_match_returns_empty() {
    let results = LucideGlyph::find("zzznomatchzzz");
    assert!(results.is_empty());
}

#[test]
fn find_whitespace_only_returns_all() {
    let results = LucideGlyph::find("   ");
    assert_eq!(results.len(), LucideGlyph::count());
}

#[test]
fn find_results_are_valid_glyphs() {
    let results = LucideGlyph::find("arrow");
    for g in &results {
        assert!(LucideGlyph::by_name(g.name()).is_some());
    }
}

// --- by_name() ---

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
fn by_name_roundtrips_all_icons() {
    for g in LucideGlyph::iter() {
        let name = g.name();
        assert_eq!(
            LucideGlyph::by_name(name),
            Some(g),
            "roundtrip failed for {name}"
        );
    }
}

// --- name() / kebab_name() ---

#[test]
fn name_roundtrips_through_by_name() {
    let icon = LucideGlyph::by_name("Activity").unwrap();
    assert_eq!(LucideGlyph::by_name(icon.name()), Some(icon));
}

#[test]
fn name_is_pascal_case() {
    for g in LucideGlyph::iter() {
        let n = g.name();
        assert!(!n.contains('-'), "name contains hyphen: {n}");
        assert!(!n.contains(' '), "name contains space: {n}");
        assert!(
            n.chars().next().is_some_and(|c| c.is_uppercase()),
            "name does not start uppercase: {n}"
        );
    }
}

#[test]
fn kebab_name_format() {
    let icon = LucideGlyph::by_name("ArrowRight").unwrap();
    assert_eq!(icon.kebab_name(), "arrow-right");
}

#[test]
fn kebab_name_single_word() {
    let icon = LucideGlyph::by_name("Search").unwrap();
    assert_eq!(icon.kebab_name(), "search");
    assert!(!icon.kebab_name().contains('-'));
}

// --- count() ---

#[test]
fn count_matches_iter() {
    assert_eq!(LucideGlyph::count(), LucideGlyph::iter().count());
}

#[test]
fn count_is_positive() {
    assert!(LucideGlyph::count() > 600);
}

#[test]
fn count_is_stable() {
    assert_eq!(LucideGlyph::count(), LucideGlyph::count());
}

// --- categories ---

#[test]
fn categories_not_empty() {
    let cats = LucideGlyph::all_categories();
    assert!(cats.len() > 30);
}

#[test]
fn all_categories_keys_are_title_case() {
    for key in LucideGlyph::all_categories().keys() {
        let first = key.chars().next().unwrap();
        assert!(first.is_uppercase(), "category not title case: {key}");
    }
}

#[test]
fn all_categories_total_count_is_sane() {
    let total: u16 = LucideGlyph::all_categories().values().sum();
    assert!(
        total as usize >= LucideGlyph::count(),
        "category sum {total} less than icon count"
    );
}

// --- data integrity ---

#[test]
fn all_icons_have_svg() {
    for g in LucideGlyph::iter() {
        let svg = g.svg();
        assert!(!svg.is_empty(), "empty svg for {}", g.name());
        assert!(svg.contains('<'), "svg has no elements for {}", g.name());
    }
}

#[test]
fn all_icons_have_at_least_one_category() {
    for g in LucideGlyph::iter() {
        assert!(
            g.categories().count() > 0,
            "no category for {}",
            g.name()
        );
    }
}

#[test]
fn contributors_no_empty_strings() {
    for g in LucideGlyph::iter() {
        for c in g.contributors() {
            assert!(!c.is_empty(), "empty contributor for {}", g.name());
        }
    }
}

#[test]
fn icon_metadata_accessible() {
    let icon = LucideGlyph::by_name("Activity").unwrap();
    assert!(!icon.svg().is_empty());
    assert!(icon.tags().count() > 0);
    assert!(icon.categories().count() > 0);
}

#[test]
fn glyph_trait_returns_svg() {
    let icon = LucideGlyph::by_name("Search").unwrap();
    let svg = icon.svg();
    assert!(svg.contains("<"));
    assert!(svg.contains("path"));
}
