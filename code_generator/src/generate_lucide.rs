use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use convert_case::{Case, Casing};
use scraper::{ElementRef, Html, Selector};

mod compress;
use compress::compress_string;

mod cargo;
use cargo::CargoToml;

fn main() {

    let icon_path: &Path = Path::new("../lucide/icons");
    let dest_path: &Path = Path::new("../lucide_icons/src/lucide_icon_data.rs");

    // read all the svg files available in the icons folder and sort them
    let mut paths = fs::read_dir(icon_path).unwrap()
        .filter_map(|entry| {
            // let entry = entry.unwrap();
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "svg" {
                Some(path)
            } else {
                None
            }
        }).collect::<Vec<_>>();

    paths.sort();


    // create the generated_icons.rs file
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        // .append(true)
        .open(dest_path).unwrap();


    // write the imports
    writeln!(file, r#"
    use strum_macros::{{EnumProperty,EnumIter}};
    "#).expect("write imports");


    // write the icons enum header
    writeln!(file, "#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone )]").expect("write enum annotation");
    writeln!(file, "pub enum LucideIcon {{").expect("write enum header");

    // write icons and collect their names
    let entries: Vec<SvgEntry> = paths.iter().map(|path| {

        let entry = SvgEntry::new(path);

        println!("{:?}", path);

        // write feature annotation
        writeln!(file, r#"#[cfg(feature = "{}")]"#,
                 entry.feature_name).expect("write feature annotation");
        //write enum props
        writeln!(file, r#"#
[strum(props(
    svg="{}",
    categories="{}",
    tags="{}",
    contributors="{}"
))]"#,
       compress_string(entry.content().as_str()).unwrap(),
       entry.meta.categories.join(",").as_str(),
       entry.meta.tags.join(",").as_str(),
       entry.meta.contributors.join(",").as_str()).expect("write icon metadata");


        //read file
        writeln!(file, "   {},",  entry.icon_name).expect("write icon enum");

        entry

    }).collect();

    // close enum
    writeln!(file, "}}").expect("write enum footer");

    // format the generated file
    let output = Command::new("rustfmt")
        .arg(dest_path)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // update Cargo.toml in lucid_icons
    let path = "../lucide_icons/Cargo.toml";
    let mut cargo = CargoToml::load(path.to_string());
    cargo.features.clear();
    cargo.features.insert("default".to_string(),
      toml::Value::Array( entries.iter()
        .map(|entry| toml::Value::String(entry.feature_name.clone()))
        .collect::<Vec<_>>()
    ));
    entries.iter().for_each(|entry| {
        cargo.features.insert(entry.feature_name.clone(),
            toml::Value::Array(vec![]));
    });


    cargo.store(path.to_string());


}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "lowerCase")]
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

        let icon_name: String = path.file_stem().iter()
            .flat_map(|os_str| os_str.to_str())
            .map(|s| s.to_case(Case::UpperCamel))
            .collect();

        println!("{:?}", path.with_extension("json"));

        let meta: EntryMeta = serde_json::from_reader(
            fs::File::open(path.with_extension("json")
            ).unwrap()).unwrap();

        Self{ path: path.clone(),
            icon_name: icon_name.clone(),
            feature_name: icon_name.to_case(Case::Snake),
            meta,
        }
    }

    fn content(&self) -> String {
        fs::read_to_string(&self.path).iter()
            .map( |s| html_children_only(s.to_string()).replace("\n", ""))
            .collect()
    }

}


// extract svg children
fn html_children_only(svg_content: String ) -> String {

    let html = Html::parse_fragment(svg_content.as_str());

    let svg = html.select(&Selector::parse("svg").unwrap()).next().unwrap();
    "\n".to_owned() + &svg.children()
        .filter_map(|node| ElementRef::wrap(node))
        .map(|el| el.html())
        .collect::<Vec<_>>()
        .join("\n")
}

