#[macro_use]
extern crate yamlette;

use std::io;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

enum Section {
    Border(String),
    Background(String),
    Text(String),
    Indicator(String),
}

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

        yamlette!(read; contents; [[
            {
                "meta" => {
                    "description" => (desc:&str)
                }
            }
        ]]);

        let desc = match desc {
            Some(d) => d,
            None => "No available description",
        };
        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(_s) => String::new(),
        };
        println!("\t{0: <20} {1: <5} {2: <100}", name, "-->", desc);
    }
    Ok(())
}

fn format_theme(theme: String) -> String {
    let result = "#".repeat(5) + " i3themes configuration " + &"#".repeat(5);

    let contents = match file_contents(&theme) {
        Ok(c) => c,
        Err(e) => {
            println!("Error accesing the theme. Found following error: \n{}\n", e);
            return result;
        }
    };
    yamlette!(read; contents; [[
        {
            "window_colors" => {
                "focused" => {
                    "border" => (w_fc_border:&str)
                }
            }
        }
    ]]);
    let result = match w_fc_border {
        Some(s) => result + s,
        None => result,
    };
    result
}

fn file_contents(path: &str) -> Result<String, Box<Error>> {
    let mut contents = String::new();
    let mut f = File::open(&path)?;
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

