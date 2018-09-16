extern crate yaml_rust;
extern crate getopts;

mod yaml;
mod config_file;

use std::io;
use std::fs;
use yaml_rust::Yaml;
use getopts::Options;

pub fn run(input: String, output: String, theme: String) {
    let path = format!("themes/{}", theme);
    let theme = yaml::load_yaml(&path).unwrap_or_else(|_e| {
            println!("Error loading the theme, try again or submit a bug report");
            vec![Yaml::BadValue]
    });
    let theme = &theme[0];

    println!("Input: {} --- Output: {}\n", input, output);
    if let Some(s) = yaml::theme_vars(theme) {
        println!("{}\n", s);
    }
    println!("{}\n", yaml::window_theme(theme));
    println!("{}\n", yaml::bar_theme(theme));

    config_file::bar_block("/home/lucas/.config/i3/config");
}

pub fn help(opts: Options) {
    println!("{} \n\n{}", "Usage:", opts.usage("i3themes <theme> [options]"));
}

pub fn version() {
    println!("Version: 0.1.0");
}

pub fn to_theme() {
    println!("ToDo");
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

        let theme = match yaml::load_yaml(path) {
            Ok(y) => y,
            Err(_e) => continue,
        };

        let desc = yaml::get_yaml_str(&theme[0], "meta", "description", "")
                    .unwrap_or("Description not found".to_string());

        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(_s) => continue,
        };
        println!("\t{0: <20} {1: <5} {2: <100}", name, "-->", desc);
    }
    Ok(())
}

