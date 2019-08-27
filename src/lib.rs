extern crate getopts;

mod theme;
mod i3config;

use std::io;
use std::fs;
use std::process;
use getopts::Options;

const THEMES_DIR: &str = "themes";

/// Apply a theme to a specified i3wm configuration file
///
/// * `input`  - Path to input configuration file
/// * `output` - Path to output configuration file
/// * `theme`  - Path to theme to be applied
///
pub fn run(input: String, output: String, theme: String) {
    let path = format!("{}/{}", THEMES_DIR, theme);
    let theme = theme::load(&path).unwrap_or_else(|e| {
            println!("Error loading the theme, try again or submit a bug report");
            println!("{:?}", e);
            process::exit(1);
    });

    match i3config::build_config(&input, theme) {
        Ok(s) => {
            if let Err(e) = fs::write(output, s) {
                println!("Error when writing file: {}", e);
            }
        }
        Err(e) => println!("Error when opening input file: {} \nInput file: {}", e, input),
    }
}

/// Extract theme as yaml
///
/// * `input`  - Path to input configuration file
/// * `output` - Path to output theme file
///
pub fn to_theme(input: String, output: String) {
    match i3config::build_theme(&input) {
        Ok(s) => {
            println!("{}", s);
        }
        Err(e) => println!("Error when opening input file: {} \nInput file: {}", e, input),
    }
}

/// List possible themes
///
pub fn list() -> io::Result<()> {
    let default = "Description not found".to_owned();
    println!("Available themes:\n");

    for entry in fs::read_dir(THEMES_DIR)? {
        let entry = entry?;
        let path = entry.path();
        let path = match path.to_str() {
            Some(s) => s,
            None => continue,
        };

        if let Ok(theme) = theme::load(path) {
            if let Some(meta) = theme.meta {
                let desc = meta.get("description").unwrap_or(&default);
                let name = match entry.file_name().into_string() {
                    Ok(s) => s,
                    Err(_s) => continue,
                };
                println!("\t{0: <20} {1: <5} {2: <100}", name, "-->", desc);
            }
        };
    }
    Ok(())
}


/// Print help message
///
/// * `opts` - Command line options
///
pub fn help(opts: Options) {
    println!("{} \n\n{}", "Usage:", opts.usage("i3themes [<theme>] [options]"));
}

/// Print version
///
pub fn version() {
    println!("Version: 0.1.0");
}
