extern crate yaml_rust;

mod yaml;

use std::io;
use std::fs;
use std::fs::File;
use std::error::Error;
use yaml::get_yaml_str;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

pub fn run(input: String, output: String, theme: String) {
    let path = format!("themes/{}", theme);
    println!("Input: {} --- Output: {} --- Theme: {}", input, output, format_theme(path));
}

pub fn list() -> io::Result<()> {
    println!("Available themes:\n");

    for entry in fs::read_dir("themes")? {
        let entry = entry?;
        let path = entry.path();
        let path = match path.to_str() {
            Some(s) => s,
            None => continue,
        };

        let contents = match file_contents(path) {
            Ok(c) => c,
            Err(_e) => continue,
        };
        let loader = &YamlLoader::load_from_str(&contents);
        
        let theme = match loader {
            Ok(ym) => &ym[0],
            Err(_e) => continue,
        };

        let desc = match get_yaml_str(theme, "meta", "description", ""){
            Some(s) => s,
            None => "Description not found".to_string(),
        };

        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(_s) => continue,
        };
        println!("\t{0: <20} {1: <5} {2: <100}", name, "-->", desc);
    }
    Ok(())
}

fn format_theme(theme: String) -> String {
    let padding = "#".repeat(8);
    let result = format!("{0} {1} {0}\n", padding, "i3themes configuration");

    let contents = match file_contents(&theme) {
        Ok(c) => c,
        Err(e) => {
            println!("Error accesing the theme. Found following error: \n{}\n", e);
            return "".to_string();
        }
    };
    result
}

fn file_contents(path: &str) -> Result<String, Box<Error>> {
    let mut contents = String::new();
    let mut f = File::open(&path)?;
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
