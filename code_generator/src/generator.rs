use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use convert_case::{Case, Casing};

const HEADER: &str = r#"
use leptos::*;

const DEFAULT_SIZE: u16 = 24;
const DEFAULT_FILL: &str = "none";
const DEFAULT_STROKE: &str = "black";
const DEFAULT_STROKE_WIDTH: u16 = 2;
const DEFAULT_SCHEMA: &str = "http://www.w3.org/2000/svg";"#;



fn main() {

    let icon_path: &Path = Path::new("../lucide/icons");

    // read all the svg files available in the icons folder and sort them
    let mut entries = fs::read_dir(icon_path).unwrap()
        .filter_map(|entry| {
            // let entry = entry.unwrap();
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "svg" {
                Some(path)
            } else {
                None
            }
        }).collect::<Vec<_>>();
    entries.sort();

    println!("Count:{}", entries.len());


    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        // .append(true)
        .open("../lucide_icons/src/generated_icons.rs").unwrap();

    writeln!(file, "{}", HEADER).expect("write header");

    entries.iter().for_each(|path| {
        // println!("{:?}", path);
        let icon_name = path.file_stem()
                                   .unwrap()
                                   .to_str().unwrap().to_case(Case::UpperCamel) + "Icon";
        println!("{:?} --> {}",  path, icon_name);

        //read file
        let content = fs::read_to_string(&path).unwrap();
        writeln!(file, r#"

#[allow(unused)]
# [component]
pub fn {}(
    cx: Scope,
    #[prop(default = DEFAULT_SIZE)]
    size: u16,
    #[prop(default = DEFAULT_FILL.to_string())]
    fill: String,
    #[prop(default = DEFAULT_STROKE.to_string())]
    stroke: String,
    #[prop(default = DEFAULT_STROKE_WIDTH)]
    stroke_width: u16,
) -> impl IntoView {{

    view! {{ cx,
            {}
        }}
}}"#, icon_name, content).expect("write icon");

    });


}