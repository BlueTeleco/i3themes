
mod theme;
mod i3config;

use std::io;
use std::fs;
use std::process;

const THEMES_DIR: &str = "themes";

/// Apply a theme to a specified i3wm configuration file
///
/// * `config` - Path to configuration file
/// * `theme`  - Path to theme to be applied
///
pub fn change(config: &str, theme: &str) {
    let path = format!("{}/{}", THEMES_DIR, theme);
    let theme = theme::load(&path).unwrap_or_else(|_e| {
            println!("Error loading the theme, try again or submit a bug report. The selected theme may not be installed.");
            process::exit(1);
    });

    match i3config::build_config(&config, theme) {
        Ok(s) => {
            if let Err(_e) = fs::write(config, s) {
                println!("Error when writing to file: {}", config);
            }
        }
        Err(_e) => println!("Error when opening config file: {}. File may not exist.", config),
    }
}

/// Extract theme as yaml
///
/// * `config` - Path to configuration file
/// * `output` - Path to output theme file
///
pub fn extract(config: &str, output: Option<&str>) {
    match i3config::build_theme(&config) {
        Ok(s) => {
            if let Some(output) = output {
                if let Err(_e) = fs::write(output, s) {
                    println!("Error when writing to file: {}", output);
                }
            } else {
                println!("{}", s);
            }
        }
        Err(_e) => println!("Error when opening config file: {}. File may not exist.", config),
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

/// Installs the provided theme.
///
/// * `theme`  - Path to theme to be installed.
///
pub fn install(theme: &str) {
    println!("Installing {}", theme);
}
