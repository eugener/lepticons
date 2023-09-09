use std::{env, fs};
use std::io;
use std::path::{Path, PathBuf};

fn list_files(dir: &Path) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            files.push(String::from(entry.path().to_string_lossy()));
        }
    }

    Ok(files)
}

pub fn split_camel_case(s: &str) -> String {
    let mut result = String::new();
    for (i, char) in s.char_indices() {
        if char.is_uppercase() || char.is_numeric() {
            if i != 0 { // Don't prefix the first word with a dash
                result.push('-');
            }
        }
        result.push(char.to_ascii_lowercase());
    }
    result
}

fn dash_to_camel(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join("")
}


pub fn list_icons() {
    match env::current_dir() {
        Ok(path) => {
            println!("Current directory: {}", path.display());
            let mut dir = PathBuf::from(path);
            dir.push("lucide");
            dir.push("icons");
            let files = list_files(&dir).unwrap();
            let mut names = files.iter()
                .filter(|f| f.ends_with(".svg"))
                .map(|f|
                    return f.split("/").last().unwrap().split(".").next().unwrap()
                ).collect::<Vec<_>>();

            // let mut names = files.collect::<Vec<_>>();
            names.sort();
            for file in names {
                // if file.ends_with(".svg") {
                //     let name = file.split("/").last().unwrap().split(".").next().unwrap();
                    println!("#[strum(props(File=\"{}\"))]", file);
                    println!("{},", dash_to_camel(file));
                // }
            }
        }
        Err(e) => println!("Couldn't get current directory: {}", e),
    }
}