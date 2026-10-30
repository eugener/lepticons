use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use convert_case::{Case, Casing};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

mod cargo;
mod metadata;
use cargo::CargoToml;
use metadata::EntryMeta;

fn main() {
    let icon_path = Path::new("../lucide/icons");
    let gen_path = Path::new("../lepticons-data");
    let dest_path = gen_path.join("src").join("lucide_icon_data.rs");
    let cargo_path = gen_path.join("Cargo.toml");
    let facade_cargo_path = Path::new("../lepticons/Cargo.toml");

    // read all the svg files available in the icons folder and sort them by path
    let mut paths = fs::read_dir(icon_path)
        .unwrap()
        .filter_map(|entry| {
            entry
                .ok()
                .filter(|e| {
                    e.path().is_file()
                        && e.path().extension().is_some_and(|ext| ext == "svg")
                })
                .map(|e| e.path())
        })
        .collect::<Vec<_>>();
    paths.sort();

    // Lucide metadata can reuse values from other icons and tag groups;
    // resolve those references before generating anything.
    let mut metas: BTreeMap<String, EntryMeta> = paths
        .iter()
        .map(|p| (icon_stem(p).to_string(), metadata::read_icon(p)))
        .collect();
    metadata::resolve_all(
        &mut metas,
        &metadata::read_groups(&icon_path.with_file_name("groups")),
    );

    // prepare the generated_icons.rs file
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(dest_path.clone())
        .expect("open file for code generation");

    // write the imports
    writeln!(
        file,
        r#"
    use strum_macros::{{EnumProperty,EnumIter,EnumString,IntoStaticStr}};
    "#
    )
    .expect("write imports");

    // write the icons enum header
    writeln!(
        file,
        "#[derive(EnumIter, EnumProperty, EnumString, IntoStaticStr, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]"
    )
    .expect("write enum annotation");
    writeln!(file, "pub enum LucideGlyph {{").expect("write enum header");

    let decimal_re = Regex::new(r"(\d+\.\d{2})\d+").unwrap();
    let escape = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");

    // write icon's enum entries and collect their names
    let entries: Vec<SvgEntry> = paths
        .iter()
        .map(|path| {
            let meta = metas.remove(icon_stem(path)).expect("metadata for icon");
            let entry = SvgEntry::new(path, meta);

            println!("{:?}", path);

            let svg_content = entry.content();
            // Truncate decimals to 2 places
            let svg_content = decimal_re.replace_all(&svg_content, "$1").to_string();
            // Escape backslashes and double quotes for Rust string literal
            let svg_escaped = escape(&svg_content);

            writeln!(file, "{}", feature_cfg(&entry.meta.categories))
                .expect("write feature annotation");

            //write enum props
            writeln!(
                file,
                r#"#
            [strum(props(
                svg="{}",
                categories="{}",
                tags="{}",
                contributors="{}"
            ))]"#,
                svg_escaped,
                escape(&entry.meta.categories.join(",")),
                escape(&entry.meta.tags.join(",")),
                escape(&entry.meta.contributors.join(",")),
            )
            .expect("write icon metadata");

            //read enum name
            writeln!(file, "   {},", entry.icon_name).expect("write icon enum");

            entry
        })
        .collect();

    // close enum
    writeln!(file, "}}").expect("write enum footer");

    write_alias_consts(&mut file, &entries);

    format_code(&dest_path);

    // Refresh feature tables in both crates so categories stay in sync.
    update_cargo_features(cargo_path.display().to_string(), &entries, None);
    update_cargo_features(
        facade_cargo_path.display().to_string(),
        &entries,
        Some("lepticons-data"),
    );
}

/// Returns an icon's kebab-case name, e.g. `arrow-right` for `arrow-right.svg`.
fn icon_stem(path: &Path) -> &str {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| panic!("icon file name {:?}", path))
}

struct SvgEntry {
    path: PathBuf,
    icon_name: String,
    meta: EntryMeta,
}

impl SvgEntry {
    fn new(path: &Path, meta: EntryMeta) -> Self {
        let icon_name = icon_stem(path).to_case(Case::UpperCamel);

        // check for empty categories
        if meta.categories.is_empty() {
            println!(">>>WARNING: {} has no categories", icon_name);
        }

        Self {
            path: path.to_path_buf(),
            icon_name,
            meta,
        }
    }

    fn content(&self) -> String {
        let raw = fs::read_to_string(&self.path)
            .unwrap_or_else(|_| panic!("read svg file {:?}", self.path));
        html_children_only(raw, &self.path).replace('\n', "")
    }
}

/// Returns the `cfg` attribute that gates an icon on its category features.
/// Icons with no category are always included.
fn feature_cfg(categories: &[String]) -> String {
    let features: Vec<String> = categories
        .iter()
        .map(|c| format!(r#"feature = "{}""#, c.to_case(Case::Snake)))
        .collect();
    match features.as_slice() {
        [] => "// no category".to_string(),
        [one] => format!("#[cfg({one})]"),
        many => format!("#[cfg(any({}))]", many.join(", ")),
    }
}

/// Writes one associated constant per Lucide alias, so code that uses an
/// icon's old name still compiles after Lucide renames the icon.
fn write_alias_consts(file: &mut fs::File, entries: &[SvgEntry]) {
    let mut names: HashSet<String> = entries.iter().map(|e| e.icon_name.clone()).collect();

    writeln!(
        file,
        "#[allow(non_upper_case_globals)]\nimpl LucideGlyph {{"
    )
    .expect("write impl header");
    for entry in entries {
        for alias in &entry.meta.aliases {
            let const_name = alias.name.to_case(Case::UpperCamel);
            // e.g. alias `arrow-down-01` of `arrow-down-0-1`: same Rust name
            if const_name == entry.icon_name {
                continue;
            }
            assert!(
                names.insert(const_name.clone()),
                "alias `{}` of `{}` clashes with another icon or alias",
                alias.name,
                entry.icon_name
            );
            writeln!(file, "{}", feature_cfg(&entry.meta.categories))
                .expect("write feature annotation");
            writeln!(
                file,
                "/// Lucide alias `{}` of [`LucideGlyph::{}`].",
                alias.name, entry.icon_name
            )
            .expect("write alias doc");
            if alias.deprecated {
                writeln!(
                    file,
                    r#"#[deprecated(note = "renamed to `LucideGlyph::{}`")]"#,
                    entry.icon_name
                )
                .expect("write deprecation");
            }
            writeln!(
                file,
                "pub const {}: Self = Self::{};",
                const_name, entry.icon_name
            )
            .expect("write alias const");
        }
    }
    writeln!(file, "}}").expect("write impl footer");
}

/// Extracts the children of an svg element as a string.
fn html_children_only(svg_content: String, path: &Path) -> String {
    let html = Html::parse_fragment(svg_content.as_str());

    let svg = html
        .select(&Selector::parse("svg").unwrap())
        .next()
        .unwrap_or_else(|| panic!("no <svg> element in {:?}", path));
    svg.children()
        .filter_map(ElementRef::wrap)
        .map(|el| el.html())
        .collect::<Vec<_>>()
        .join("")
}

/// Formats the code of a file using rustfmt.
fn format_code(file_path: &Path) {
    let output = Command::new("rustfmt")
        .arg(file_path)
        .output()
        .expect("Failed to execute `rustfmt` command");

    if !output.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Rewrites the `[features]` table.
///
/// When `forward_to` is `None`, each category becomes an empty feature
/// (`category = []`). When set to e.g. `Some("lepticons-data")`, the value
/// becomes `["lepticons-data/<category>"]` so the facade crate forwards to
/// the data crate. The `default` feature always lists every category.
fn update_cargo_features(path: String, entries: &[SvgEntry], forward_to: Option<&str>) {
    let mut all_categories = BTreeSet::new();
    for entry in entries {
        for cat in &entry.meta.categories {
            all_categories.insert(cat.to_case(Case::Snake));
        }
    }

    let mut cargo = CargoToml::load(path.clone());
    let features = cargo.features();
    let prev_count = features.keys().filter(|k| *k != "default").count();
    features.clear();

    // `default` always lists the local feature names; each local feature
    // either is empty (data crate) or forwards (facade crate).
    features.insert(
        "default".to_string(),
        toml::Value::Array(
            all_categories
                .iter()
                .map(|c| toml::Value::String(c.clone()))
                .collect(),
        ),
    );

    for cat in &all_categories {
        let value = match forward_to {
            Some(prefix) => toml::Value::Array(vec![toml::Value::String(format!("{prefix}/{cat}"))]),
            None => toml::Value::Array(vec![]),
        };
        features.insert(cat.clone(), value);
    }

    let new_count = all_categories.len();
    println!(
        ">>> {}: Categories {} (delta: {:+}), Icons: {}",
        path,
        new_count,
        new_count as isize - prev_count as isize,
        entries.len(),
    );

    cargo.store(path);
}
