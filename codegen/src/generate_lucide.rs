use std::collections::BTreeSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use convert_case::{Case, Casing};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

mod cargo;
use cargo::CargoToml;

fn main() {
    let icon_path = Path::new("../lucide/icons");
    let gen_path = Path::new("../lepticons");
    let dest_path = gen_path.join("src").join("lucide_icon_data.rs");
    let cargo_path = gen_path.join("Cargo.toml");

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
    use strum_macros::{{EnumProperty,EnumIter,EnumString}};
    "#
    )
    .expect("write imports");

    // write the icons enum header
    writeln!(
        file,
        "#[derive(EnumIter, EnumProperty, EnumString, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]"
    )
    .expect("write enum annotation");
    writeln!(file, "pub enum LucideGlyph {{").expect("write enum header");

    let decimal_re = Regex::new(r"(\d+\.\d{2})\d+").unwrap();
    let escape = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");

    // write icon's enum entries and collect their names
    let entries: Vec<SvgEntry> = paths
        .iter()
        .map(|path| {
            let entry = SvgEntry::new(path);

            println!("{:?}", path);

            let svg_content = entry.content();
            // Truncate decimals to 2 places
            let svg_content = decimal_re.replace_all(&svg_content, "$1").to_string();
            // Escape backslashes and double quotes for Rust string literal
            let svg_escaped = escape(&svg_content);

            // write feature annotation based on categories
            let feature_cats: Vec<String> = entry
                .meta
                .categories
                .iter()
                .map(|c| c.to_case(Case::Snake))
                .collect();

            if feature_cats.is_empty() {
                // Icons with no category are always included
                writeln!(file, "// no category").expect("write comment");
            } else if feature_cats.len() == 1 {
                writeln!(file, r#"#[cfg(feature = "{}")]"#, feature_cats[0])
                    .expect("write feature annotation");
            } else {
                let conditions: Vec<String> = feature_cats
                    .iter()
                    .map(|c| format!(r#"feature = "{}""#, c))
                    .collect();
                writeln!(file, "#[cfg(any({}))]", conditions.join(", "))
                    .expect("write feature annotation");
            }

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

    format_code(&dest_path);

    // update Cargo.toml in lucid_icons
    update_cargo_features(cargo_path.display().to_string(), &entries);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct EntryMeta {
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    contributors: Vec<String>,
}

struct SvgEntry {
    path: PathBuf,
    icon_name: String,
    meta: EntryMeta,
}

impl SvgEntry {
    fn new(path: &Path) -> Self {
        let icon_name: String = path
            .file_stem()
            .iter()
            .flat_map(|os_str| os_str.to_str())
            .map(|s| s.to_case(Case::UpperCamel))
            .collect();

        let meta: EntryMeta = serde_json::from_reader(
            fs::File::open(path.with_extension("json"))
                .unwrap_or_else(|_| panic!("open json for {:?}", path)),
        )
        .unwrap_or_else(|_| panic!("parse json for {:?}", path));

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

fn update_cargo_features(path: String, entries: &[SvgEntry]) {
    // Collect all unique categories
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

    // default enables all categories
    features.insert(
        "default".to_string(),
        toml::Value::Array(
            all_categories
                .iter()
                .map(|c| toml::Value::String(c.clone()))
                .collect(),
        ),
    );

    // each category is an empty feature
    for cat in &all_categories {
        features.insert(cat.clone(), toml::Value::Array(vec![]));
    }

    let new_count = all_categories.len();
    println!(
        ">>> Categories: {} (delta: {:+}), Icons: {}",
        new_count,
        new_count as isize - prev_count as isize,
        entries.len(),
    );

    cargo.store(path);
}
