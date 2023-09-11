use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use convert_case::{Case, Casing};
use scraper::{ElementRef, Html, Selector};


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


    write!(file, r#"pub struct IconType<'a> {{
        pub content: &'a str,
    }}
    "#).expect("write icon type");

    entries.iter().for_each(|path| {
        // println!("{:?}", path);
        let icon_name = path.file_stem()
                                   .unwrap()
                                   .to_str().unwrap().to_case(Case::UpperCamel);
        println!("{:?} --> {}",  path, icon_name);

        //read file
        let content = fs::read_to_string(&path).unwrap();

        writeln!(file, "\npub const {}: IconType = IconType{{ \
        \n content: r#\"{}\"#,\
        \n}};",
             icon_name.to_case(Case::UpperSnake),
             only_children(content)).expect("write icon");

    });


}

fn only_children(svg_content: String ) -> String {

    let html = Html::parse_fragment(svg_content.as_str());

    let svg = html.select(&Selector::parse("svg").unwrap()).next().unwrap();
    "\n".to_owned() + &svg.children()
            .filter_map(|node| ElementRef::wrap(node))
            .map(|el| el.html())
            .collect::<Vec<_>>()
            .join("\n")
}



