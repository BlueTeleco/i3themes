extern crate yaml_rust;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use yaml_rust::{YamlLoader, Yaml};

pub fn load_yaml(path: &str) -> Result<Vec<Yaml>, Box<Error>> {
    let contents = file_contents(path)?;
    Ok(YamlLoader::load_from_str(&contents)?)
}

pub fn get_yaml_str(yaml: &Yaml, first: &str, second: &str, third: &str) -> Option<String> {
    let h = yaml.as_hash()?;
    if first != "" {
        let value = h.get(&Yaml::from_str(first))?;
        match value {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::Hash(_h) => get_yaml_str(value, second, third, ""),
            _ => None,
        }
    } else {
        None
    }
}


fn file_contents(path: &str) -> Result<String, Box<Error>> {
    let mut contents = String::new();
    let mut f = File::open(&path)?;
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
