extern crate yaml_rust;

mod yaml;

use std::io;
use std::fs;
use yaml::*;

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

        let theme = match load_yaml(path) {
            Ok(y) => y,
            Err(_e) => continue,
        };

        let desc = match get_yaml_str(&theme[0], "meta", "description", "") {
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
    result
}
