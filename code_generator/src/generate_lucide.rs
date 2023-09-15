use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use convert_case::{Case, Casing};
use scraper::{ElementRef, Html, Selector};

fn main() {

    let icon_path: &Path = Path::new("../lucide/icons");

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
        .open("../lucide_icons/src/lucide_icon_data.rs").unwrap();

    // write the imports
    writeln!(file, "use strum_macros::{{EnumProperty,EnumIter}};").expect("write imports");
    // writeln!(file, "use strum::{{EnumProperty,IntoEnumIterator}};").expect("write imports");
    // writeln!(file, "\n").expect("write imports");
    writeln!(file, "use core::fmt;\n").expect("write imports");
    writeln!(file, "use fmt::Result;\n").expect("write imports");

    writeln!(file, "#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone )]").expect("write enum annotation");
    writeln!(file, "pub enum LucideIcon {{").expect("write enum header");

    // write icons and collect their names
    let entries: Vec<SvgEntry> = paths.iter().map(|path| {

        let entry = SvgEntry::new(path);

        // println!("{:?}", path);
        println!("{:?} --> {} --> {}",  path, entry.icon_name, entry.feature_name);
        // println!("\"{}\",", feature_name);

        //read file
        writeln!(file, "   {},",  entry.icon_name).expect("write icon enum");

        entry

    }).collect();

    writeln!(file, "}}").expect("write enum footer");

    entries.iter().for_each(|entry| {

        writeln!(file, "const {}: &str = r#\"{}\"#;\n",
            entry.const_name,
            entry.content()).expect("write icon const");
    });

    writeln!(file, r#"impl LucideIcon {{
        pub fn svg(&self) -> String {{
           match self {{
    "#).expect("write impl header");
    entries.iter().for_each(|entry| {
        writeln!(file, " &Self::{} => {}.to_string(),",
            entry.icon_name,
            entry.const_name).expect("write icon const");

    });
    writeln!(file, "}}\n}}\n}}").expect("write enum footer");


}

struct SvgEntry {
    path: PathBuf,
    icon_name: String,
    feature_name: String,
    const_name: String,
}

impl SvgEntry {

    fn new(path: &PathBuf) -> Self {

        let icon_name = path.file_stem()
            .unwrap()
            .to_str().unwrap().to_case(Case::UpperCamel);

        Self{ path: path.clone(),
            icon_name: icon_name.clone(),
            feature_name: icon_name.to_case(Case::Snake),
            const_name: icon_name.to_case(Case::UpperSnake),
        }
    }

    fn content(&self) -> String {
        only_children(fs::read_to_string(&self.path).unwrap()
            // .replace("\"","\\\"")
            .replace("\n", ""))
    }


}


// extract svg children
fn only_children(svg_content: String ) -> String {

    let html = Html::parse_fragment(svg_content.as_str());

    let svg = html.select(&Selector::parse("svg").unwrap()).next().unwrap();
    "\n".to_owned() + &svg.children()
        .filter_map(|node| ElementRef::wrap(node))
        .map(|el| el.html())
        .collect::<Vec<_>>()
        .join("\n")
}

