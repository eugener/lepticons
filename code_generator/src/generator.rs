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

    // println!("Count:{}", entries.len());


    // create the generated_icons.rs file
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        // .append(true)
        .open("../lucide_icons/src/generated_icons.rs").unwrap();


    // write the imports
    write!(file, "use crate::IconType;\n").expect("write icon type");

    // write icons and collect their names
    let names: Vec<String> = entries.iter().map(|path| {
        // println!("{:?}", path);
        let icon_name = path.file_stem()
                                   .unwrap()
                                   .to_str().unwrap().to_case(Case::UpperSnake);
        let feature_name = icon_name.to_case(Case::Snake);
        // println!("{:?} --> {}",  path, icon_name);
        println!("\"{}\",", feature_name);


        //read file
        let content = fs::read_to_string(&path).unwrap();

        writeln!(file, "\n#[cfg(feature = \"{}\")]\npub const {}: IconType = IconType{{ \
        \n content: r#\"{}\"#,\
        \n name: \"{}\",\
        \n}};",
            feature_name,
            icon_name,
            only_children(content),
            icon_name).expect("write icon");

        icon_name

    }).collect();

    // write all icons array
    writeln!(
        file,
        "\n#[cfg(feature = \"all_icons\")]\npub const ALL_ICONS: [IconType; {}] = [{}];",
        names.len(),
        names.join(",\n")
    ).expect("write icon");


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


