extern crate i3themes;

extern crate dirs;

#[macro_use]
extern crate clap;
use clap::App;

use std::process;
use std::path::PathBuf;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let xdg_location = home_subfile(".config/i3/config");
    let i3h_location = home_subfile(".i3/config");
    let etc_location = PathBuf::from("/etc/i3/config");

    if let Some(matches) = matches.subcommand_matches("change") {
        let config = matches.value_of("config").unwrap_or(find_config(vec![&xdg_location, &i3h_location, &etc_location]));
        let theme = matches.value_of("theme").unwrap();
        i3themes::change(config, theme)
    }

    if let Some(matches) = matches.subcommand_matches("extract") {
        let config = matches.value_of("config").unwrap_or(find_config(vec![&xdg_location, &i3h_location, &etc_location]));
        let output = matches.value_of("output");
        i3themes::extract(config, output);
    }

    if let Some(_m) = matches.subcommand_matches("list") {
        if let Err(_e) = i3themes::list() {
            println!("The themes have not been installed correctly.");
        }
    }

    if let Some(matches) = matches.subcommand_matches("install") {
        let theme = matches.value_of("theme");
        i3themes::install(theme.unwrap());
    }
}

/// Find system configuration file in use.
///
/// * `configs` - Vector with the possible paths of the config file.
///
fn find_config<'a>(configs: Vec<&'a PathBuf>) -> &'a str {
    for conf in configs {
        if conf.exists() {
            return conf.to_str().unwrap();
        }
    }
    println!("No config file found. See help menu for more options.");
    process::exit(1);
}

/// Build a path for a file in the home directory.
///
/// * `file` - File found in the home directory.
///
fn home_subfile(file: &str) -> PathBuf {
    if let Some(mut home) = dirs::home_dir() {
        home.push(file);
        home
    } else {
        PathBuf::from("~")
    }
}
