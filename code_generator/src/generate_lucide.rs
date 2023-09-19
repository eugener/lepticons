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
    use strum::EnumProperty;
    use strum_macros::{{EnumProperty,EnumIter}};
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

        println!("{:?}", path);

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


    // write static const for each icon
    // entries.iter().for_each(|entry| {
    //
    //     writeln!(file, "const {}: &'static str = r#\"{}\"#;\n",
    //         entry.const_name,
    //         compress_string(entry.content().as_str()).unwrap()).expect("write icon const");
    //         // entry.content()).expect("write icon const");
    // });

    // write impl for LucideIcon allowing to get the svg content
    writeln!(file, r#"impl LucideIcon {{"#).expect("write impl header");

    writeln!( file, r#"

    fn decompress(&self, input: &str) -> String {{

    let input = base64::decode(input).unwrap();
    let mut decoder = ZlibDecoder::new(input.as_slice());
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).expect("decompress");
    decompressed
    }}

    pub fn svg(&self) -> String {{
        self.decompress(self.get_str("svg").expect("get svg"))
    }}

    pub fn categories(&self) -> Vec<&str> {{
        self.get_str("categories")
            .expect("get categories")
            .split(',')
            .collect::<Vec<&str>>()
    }}

    pub fn tags(&self) -> Vec<&str> {{
        self.get_str("tags")
            .expect("get tags")
            .split(',')
            .collect::<Vec<&str>>()
    }}

    pub fn contributors(&self) -> Vec<&str> {{
        self.get_str("contributors")
            .expect("get contributors")
            .split(',')
            .collect::<Vec<&str>>()
    }}
    "#);

    // writeln!(file, r#"
    // pub fn svg(&self) -> String{{
    //        match self {{
    // "#).expect("write impl header");
    //
    // entries.iter().for_each(|entry| {
    //     writeln!(file, " &Self::{} => self.decompress({}),",
    //         entry.icon_name,
    //         entry.const_name).expect("write icon const");
    //
    // });
    writeln!(file, "}}").expect("write impl footer");

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
    const_name: String,
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
            const_name: icon_name.to_case(Case::UpperSnake),
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

