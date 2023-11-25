use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use convert_case::{Case, Casing};
use scraper::{ElementRef, Html, Selector};

use cargo::CargoToml;
use compress::compress_str;

mod cargo;
mod compress;

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
                .filter(|e| e.path().is_file() && e.path().extension().unwrap() == "svg")
                .map(|e| e.path())
        })
        .collect::<Vec<_>>();
    paths.sort();

    // prepare the generated_icons.rs file
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        // .append(true)
        .open(dest_path.clone())
        .expect("open file for code generation");

    // write the imports
    writeln!(
        file,
        r#"
    use strum_macros::{{EnumProperty,EnumIter}};
    "#
    )
    .expect("write imports");

    // write the icons enum header
    writeln!(
        file,
        "#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone )]"
    )
    .expect("write enum annotation");
    writeln!(file, "pub enum LucideGlyph {{").expect("write enum header");

    // write icon's enum entries and collect their names
    let entries: Vec<SvgEntry> = paths
        .iter()
        .map(|path| {
            let entry = SvgEntry::new(path);

            println!("{:?}", path);

            // write feature annotation
            writeln!(file, r#"#[cfg(feature = "{}")]"#, entry.feature_name)
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
                compress_str(entry.content().as_str()).unwrap(),
                entry.meta.categories.join(",").as_str(),
                entry.meta.tags.join(",").as_str(),
                entry.meta.contributors.join(",").as_str()
            )
            .expect("write icon metadata");

            //read enum name
            writeln!(file, "   {},", entry.icon_name).expect("write icon enum");

            entry
        })
        .collect();

    // close enum
    writeln!(file, "}}").expect("write enum footer");

    format_code(&mut dest_path.clone());

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
    feature_name: String,
    meta: EntryMeta,
}

impl SvgEntry {
    fn new(path: &PathBuf) -> Self {
        let icon_name: String = path
            .file_stem()
            .iter()
            .flat_map(|os_str| os_str.to_str())
            .map(|s| s.to_case(Case::UpperCamel))
            .collect();

        println!("{:?}", path.with_extension("json"));

        let meta: EntryMeta = serde_json::from_reader(
            fs::File::open(path.with_extension("json")).expect("open json"),
        )
        .expect("read json file");

        // check for empty categories
        if meta.categories.is_empty() {
            println!(">>>WARNING: {} has no categories", icon_name);
        }

        Self {
            path: path.clone(),
            icon_name: icon_name.clone(),
            feature_name: icon_name.to_case(Case::Snake),
            meta,
        }
    }

    fn content(&self) -> String {
        fs::read_to_string(&self.path)
            .iter()
            .map(|s| html_children_only(s.to_string()).replace("\n", ""))
            .collect()
    }
}

/**
 * Extracts the children/operators of an svg element as a string
 * @param svg_content the svg content
 * @returns the children of the svg element as a string
 */
fn html_children_only(svg_content: String) -> String {
    let html = Html::parse_fragment(svg_content.as_str());

    let svg = html
        .select(&Selector::parse("svg").unwrap())
        .next()
        .unwrap();
    svg.children()
        .filter_map(|node| ElementRef::wrap(node))
        .map(|el| el.html())
        .collect::<Vec<_>>()
        .join("")
}

/**
 * Formats the code of a file using rustfmt
 * @param file_path the path to the file to format
 */
fn format_code(file_path: &mut Path) {
    let output = Command::new("rustfmt")
        .arg(file_path)
        .output()
        .expect("Failed to execute `rustfmt` command");

    if !output.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn update_cargo_features(path: String, entries: &Vec<SvgEntry>) {
    let mut cargo = CargoToml::load(path.clone());
    let prev_features = cargo.features.iter().count();
    cargo.features.clear();
    cargo.features.insert(
        "default".to_string(),
        toml::Value::Array(
            entries
                .iter()
                .map(|entry| toml::Value::String(entry.feature_name.clone()))
                .collect::<Vec<_>>(),
        ),
    );
    entries.iter().for_each(|entry| {
        cargo
            .features
            .insert(entry.feature_name.clone(), toml::Value::Array(vec![]));
    });
    println!(
        ">>> New icons: {}",
        cargo.features.iter().count() - prev_features
    );

    cargo.store(path.clone());
}
