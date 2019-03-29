extern crate yaml_rust;
extern crate getopts;

mod yaml;
mod config_file;
mod theme_file;

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

    let var_theme = yaml::theme_vars(theme);
    let win_theme = yaml::window_theme(theme);
    let bar_theme = yaml::bar_theme(theme);

    match config_file::output_file(&input, var_theme, win_theme, bar_theme) {
        Ok(s) => {
            if let Err(e) = fs::write(output, s) {
                println!("Error when writing file: {}", e);
            }
        }
        Err(e) => println!("Error when opening input file: {} \nInput file: {}", e, input),
    }
}

pub fn help(opts: Options) {
    println!("{} \n\n{}", "Usage:", opts.usage("i3themes <theme> [options]"));
}

pub fn version() {
    println!("Version: 0.1.0");
}

pub fn to_theme(input: String, output: String) {
    match theme_file::output_file(&input) {
        Ok(s) => {
            println!("{}", s);
        }
        Err(e) => println!("Error when opening input file: {} \nInput file: {}", e, input),
    }
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

