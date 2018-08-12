#[macro_use]
extern crate yamlette;

use std::io;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn run(input: String, output: String, theme: String) {
    let path = format!("themes/{}", theme);
    println!("Input: {} --- Output: {} --- Theme: {:?}", input, output, path);
}

pub fn list() -> io::Result<()> {
    println!("Available themes:\n");

    for entry in fs::read_dir("themes")? {
        let entry = entry?;
        let path = entry.path();

        let mut f = match File::open(&path) {
            Ok(f)   => f,
            Err(_e) => continue,
        };
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

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
