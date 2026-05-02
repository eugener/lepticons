//! Most-recently-used icon list backed by `localStorage`.
//!
//! Stored as a JSON-array of glyph variant names (e.g.
//! `["Heart","ArrowRight","Search"]`). Names are used instead of enum
//! discriminants because the latter can shift when icons are added,
//! removed, or rearranged across `lepticons` releases. Unknown entries
//! are pruned silently on load.

use lepticons::LucideGlyph;

/// Maximum number of icons retained in the MRU list.
pub const MRU_LIMIT: usize = 8;

/// Loads the MRU list from `localStorage`. Returns an empty `Vec` when
/// there is no browser, no storage permission, no stored value, or the
/// stored value cannot be parsed. Invalid icon names are dropped.
pub fn load(storage_key: &str) -> Vec<LucideGlyph> {
    let Some(window) = web_sys::window() else {
        return Vec::new();
    };
    let storage = match window.local_storage() {
        Ok(Some(s)) => s,
        _ => return Vec::new(),
    };
    let raw = match storage.get_item(storage_key) {
        Ok(Some(v)) => v,
        _ => return Vec::new(),
    };
    parse_names(&raw)
        .into_iter()
        .filter_map(|name| LucideGlyph::by_name(&name))
        .take(MRU_LIMIT)
        .collect()
}

/// Persists the MRU list to `localStorage`. Best-effort: errors (private
/// mode, quota, missing window) are swallowed silently.
pub fn save(storage_key: &str, list: &[LucideGlyph]) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let storage = match window.local_storage() {
        Ok(Some(s)) => s,
        _ => return,
    };
    let _ = storage.set_item(storage_key, &serialize_names(list));
}

/// Pushes `icon` to the front of `list`, deduplicating prior entries
/// and capping length at [`MRU_LIMIT`].
pub fn push_into(list: &mut Vec<LucideGlyph>, icon: LucideGlyph) {
    list.retain(|g| *g != icon);
    list.insert(0, icon);
    if list.len() > MRU_LIMIT {
        list.truncate(MRU_LIMIT);
    }
}

fn parse_names(raw: &str) -> Vec<String> {
    let s = raw.trim();
    let s = s.strip_prefix('[').unwrap_or(s);
    let s = s.strip_suffix(']').unwrap_or(s);
    s.split(',')
        .map(|p| {
            p.trim()
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string()
        })
        .filter(|p| !p.is_empty())
        .collect()
}

fn serialize_names(list: &[LucideGlyph]) -> String {
    let mut s = String::with_capacity(list.len() * 16);
    s.push('[');
    for (i, g) in list.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push('"');
        s.push_str(g.name());
        s.push('"');
    }
    s.push(']');
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_handles_well_formed_json() {
        assert_eq!(
            parse_names(r#"["Heart","ArrowRight"]"#),
            vec!["Heart".to_string(), "ArrowRight".to_string()]
        );
    }

    #[test]
    fn parse_skips_empty_segments() {
        assert_eq!(parse_names("[]"), Vec::<String>::new());
        assert_eq!(parse_names(""), Vec::<String>::new());
    }

    #[test]
    fn serialize_roundtrips() {
        let list = vec![LucideGlyph::Heart, LucideGlyph::Search];
        let s = serialize_names(&list);
        assert_eq!(s, r#"["Heart","Search"]"#);
        let names = parse_names(&s);
        assert_eq!(names, vec!["Heart".to_string(), "Search".to_string()]);
    }

    #[test]
    fn push_dedups_and_caps() {
        let mut list = Vec::new();
        for _ in 0..(MRU_LIMIT + 2) {
            push_into(&mut list, LucideGlyph::Heart);
        }
        assert_eq!(list, vec![LucideGlyph::Heart]);

        push_into(&mut list, LucideGlyph::Search);
        assert_eq!(list, vec![LucideGlyph::Search, LucideGlyph::Heart]);
    }
}
