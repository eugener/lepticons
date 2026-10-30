use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;

const EXTENDS: &str = "$extends:";
const GROUP: &str = "$group:";

/// Metadata from an icon's JSON file.
#[derive(Debug, Default, serde::Deserialize)]
pub struct EntryMeta {
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub contributors: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<Alias>,
}

/// Another name for an icon, usually its name before a Lucide rename.
#[derive(Debug, serde::Deserialize)]
pub struct Alias {
    pub name: String,
    #[serde(default)]
    pub deprecated: bool,
}

#[derive(serde::Deserialize)]
struct Group {
    tags: Vec<String>,
}

/// Reads the JSON metadata next to an icon's SVG file.
pub fn read_icon(svg_path: &Path) -> EntryMeta {
    let file = fs::File::open(svg_path.with_extension("json"))
        .unwrap_or_else(|_| panic!("open json for {:?}", svg_path));
    serde_json::from_reader(file).unwrap_or_else(|_| panic!("parse json for {:?}", svg_path))
}

/// Reads `groups/<name>.json` files into a map of group name to tags.
pub fn read_groups(dir: &Path) -> HashMap<String, Vec<String>> {
    fs::read_dir(dir)
        .unwrap_or_else(|_| panic!("read groups dir {:?}", dir))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .map(|p| {
            let name = p.file_stem().and_then(|s| s.to_str()).expect("group name");
            let group: Group = serde_json::from_reader(fs::File::open(&p).expect("open group"))
                .unwrap_or_else(|_| panic!("parse group {:?}", p));
            (name.to_string(), group.tags)
        })
        .collect()
}

/// Expands Lucide's metadata markers in place, the same way Lucide's
/// `resolveMetadataExtends.ts` does:
/// - `$extends:<icon>` in tags, categories, or contributors is replaced by
///   that icon's resolved values for the same field.
/// - `$group:<name>` in tags is replaced by the group's tags.
///
/// Each list keeps its order and drops later duplicates. Panics on a cycle,
/// a missing icon or group, or a group marker outside tags.
pub fn resolve_all(metas: &mut BTreeMap<String, EntryMeta>, groups: &HashMap<String, Vec<String>>) {
    let names: Vec<String> = metas.keys().cloned().collect();
    let mut done = HashSet::new();
    for name in &names {
        resolve(name, metas, groups, &mut done, &mut Vec::new());
    }
}

fn resolve(
    name: &str,
    metas: &mut BTreeMap<String, EntryMeta>,
    groups: &HashMap<String, Vec<String>>,
    done: &mut HashSet<String>,
    stack: &mut Vec<String>,
) {
    if done.contains(name) {
        return;
    }
    assert!(
        !stack.iter().any(|n| n == name),
        "circular $extends: {} -> {name}",
        stack.join(" -> ")
    );
    let meta = metas
        .get(name)
        .unwrap_or_else(|| panic!("`{}` extends missing icon `{name}`", stack.join(" -> ")));
    let parents: Vec<String> = [&meta.tags, &meta.categories, &meta.contributors]
        .into_iter()
        .flatten()
        .filter_map(|e| e.strip_prefix(EXTENDS).map(String::from))
        .collect();

    stack.push(name.to_string());
    for parent in &parents {
        resolve(parent, metas, groups, done, stack);
    }
    stack.pop();

    let meta = &metas[name];
    let tags = expand(name, &meta.tags, |m| &m.tags, metas, Some(groups));
    let categories = expand(name, &meta.categories, |m| &m.categories, metas, None);
    let contributors = expand(name, &meta.contributors, |m| &m.contributors, metas, None);

    let meta = metas.get_mut(name).expect("icon still present");
    meta.tags = tags;
    meta.categories = categories;
    meta.contributors = contributors;
    done.insert(name.to_string());
}

/// Expands the markers in one field. Parents must already be resolved.
/// `groups` is `None` for fields where `$group:` is not allowed.
fn expand(
    name: &str,
    values: &[String],
    field: fn(&EntryMeta) -> &Vec<String>,
    metas: &BTreeMap<String, EntryMeta>,
    groups: Option<&HashMap<String, Vec<String>>>,
) -> Vec<String> {
    let mut out = Vec::new();
    for value in values {
        if let Some(parent) = value.strip_prefix(EXTENDS) {
            out.extend(field(&metas[parent]).iter().cloned());
        } else if let Some(group) = value.strip_prefix(GROUP) {
            let groups = groups.unwrap_or_else(|| panic!("`{name}` uses `{value}` outside tags"));
            let tags = groups
                .get(group)
                .unwrap_or_else(|| panic!("`{name}` uses missing group `{group}`"));
            out.extend(tags.iter().cloned());
        } else {
            out.push(value.clone());
        }
    }
    let mut seen = HashSet::new();
    out.retain(|v| seen.insert(v.clone()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| v.to_string()).collect()
    }

    fn meta(tags: &[&str], categories: &[&str]) -> EntryMeta {
        EntryMeta {
            tags: strings(tags),
            categories: strings(categories),
            ..Default::default()
        }
    }

    fn resolved(
        icons: Vec<(&str, EntryMeta)>,
        groups: &[(&str, &[&str])],
    ) -> BTreeMap<String, EntryMeta> {
        let mut metas = icons.into_iter().map(|(n, m)| (n.to_string(), m)).collect();
        let groups = groups
            .iter()
            .map(|(n, t)| (n.to_string(), strings(t)))
            .collect();
        resolve_all(&mut metas, &groups);
        metas
    }

    #[test]
    fn extends_expands_in_place_through_a_chain() {
        let metas = resolved(
            vec![
                ("alarm-clock", meta(&["alarm", "$extends:clock"], &["time"])),
                (
                    "alarm-clock-check",
                    meta(
                        &["$extends:alarm-clock", "done"],
                        &["$extends:alarm-clock", "notifications"],
                    ),
                ),
                ("clock", meta(&["time", "watch"], &["time"])),
            ],
            &[],
        );
        assert_eq!(
            metas["alarm-clock-check"].tags,
            strings(&["alarm", "time", "watch", "done"])
        );
        assert_eq!(
            metas["alarm-clock-check"].categories,
            strings(&["time", "notifications"])
        );
    }

    #[test]
    fn group_expands_in_tags() {
        let metas = resolved(
            vec![("circle-x", meta(&["circle", "$group:x"], &["shapes"]))],
            &[("x", &["x", "remove", "circle"])],
        );
        assert_eq!(metas["circle-x"].tags, strings(&["circle", "x", "remove"]));
    }

    #[test]
    fn contributors_follow_extends() {
        let mut child = meta(&["a"], &["shapes"]);
        child.contributors = strings(&["$extends:base", "someone"]);
        let mut base = meta(&["b"], &["shapes"]);
        base.contributors = strings(&["author"]);
        let metas = resolved(vec![("child", child), ("base", base)], &[]);
        assert_eq!(metas["child"].contributors, strings(&["author", "someone"]));
    }

    #[test]
    #[should_panic(expected = "circular $extends")]
    fn cycle_panics() {
        resolved(
            vec![
                ("a", meta(&["$extends:b"], &[])),
                ("b", meta(&["$extends:a"], &[])),
            ],
            &[],
        );
    }

    #[test]
    #[should_panic(expected = "extends missing icon `nope`")]
    fn missing_parent_panics() {
        resolved(vec![("a", meta(&["$extends:nope"], &[]))], &[]);
    }

    #[test]
    #[should_panic(expected = "outside tags")]
    fn group_in_categories_panics() {
        resolved(vec![("a", meta(&[], &["$group:x"]))], &[("x", &["x"])]);
    }

    #[test]
    #[should_panic(expected = "missing group `nope`")]
    fn missing_group_panics() {
        resolved(vec![("a", meta(&["$group:nope"], &[]))], &[]);
    }
}
