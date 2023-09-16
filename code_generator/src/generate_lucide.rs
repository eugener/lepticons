use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use convert_case::{Case, Casing};
use scraper::{ElementRef, Html, Selector};

mod compress;
use compress::compress_string;

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
    use core::fmt;
    use fmt::Result;
    use base64::*;
    use flate2::read::ZlibDecoder;
    use std::io::prelude::*;
    "#).expect("write imports");


    // write the icons enum header
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

    // close enum
    writeln!(file, "}}").expect("write enum footer");


    // write static const for each icon
    entries.iter().for_each(|entry| {
        writeln!(file, "const {}: &'static str = r#\"{}\"#;\n",
            entry.const_name,
            compress_string(entry.content().as_str()).unwrap()).expect("write icon const");
            // entry.content()).expect("write icon const");
    });

    // write impl for LucideIcon allowing to get the svg content
    writeln!(file, r#"impl LucideIcon {{"#).expect("write impl header");

    writeln!( file, r#"

    fn decompress(&self, input: &str) -> String {{

    use base64::decode;
    use flate2::read::ZlibDecoder;
    use std::io::prelude::*;

    let input = base64::decode(input).unwrap();
    let mut decoder = ZlibDecoder::new(input.as_slice());
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).expect("decompress");
    decompressed
}}"#);

    writeln!(file, r#"
    pub fn svg(&self) -> String{{
           match self {{
    "#).expect("write impl header");

    entries.iter().for_each(|entry| {
        writeln!(file, " &Self::{} => self.decompress({}),",
            entry.icon_name,
            entry.const_name).expect("write icon const");

    });
    writeln!(file, "}}\n}}\n}}").expect("write enum footer");

    // format the generated file
    let output = Command::new("rustfmt")
        .arg(dest_path)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

}

struct SvgEntry {
    path: PathBuf,
    icon_name: String,
    feature_name: String,
    const_name: String,
}

impl SvgEntry {

    fn new(path: &PathBuf) -> Self {

        let icon_name: String = path.file_stem().iter()
            .flat_map(|os_str| os_str.to_str())
            .map(|s| s.to_case(Case::UpperCamel))
            .collect();

        Self{ path: path.clone(),
            icon_name: icon_name.clone(),
            feature_name: icon_name.to_case(Case::Snake),
            const_name: icon_name.to_case(Case::UpperSnake),
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

