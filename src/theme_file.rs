
use std::fs::File;
use std::hash::Hash;
use std::error::Error;
use yaml_rust::{Yaml, yaml};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

trait ToYaml {
    fn to_yaml(&self) -> Yaml;
}

impl ToYaml for String {
    fn to_yaml(&self) -> Yaml {
        Yaml::from_str(self)
    }
}

impl ToYaml for &str {
    fn to_yaml(&self) -> Yaml {
        Yaml::from_str(self)
    }
}

impl<T> ToYaml for HashMap<T,T> where T: ToYaml + Eq + Hash {
    fn to_yaml(&self) -> Yaml {
        let mut hash = yaml::Hash::new();
        for (key, val) in (*self).iter() {
            hash.insert(key.to_yaml(), val.to_yaml());
        }
        Yaml::Hash(hash)
    }
}

impl ToYaml for Yaml {
    fn to_yaml(&self) -> Yaml {
        self.to_owned()
    }
}

/// Create the new yaml theme file from the selected
/// configuration file.
///
/// * `path`  - Path to the configuration file to modify
///
pub fn output_file(path: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(path)?;
    let mut hash = yaml::Hash::new();

    hash.insert("meta".to_yaml(), meta().to_yaml());
    hash.insert("colors".to_yaml(), theme_vars(&file).to_yaml());

    file.seek(SeekFrom::Start(0)).unwrap();
    let windows = window_colors(&file);
    println!("{:#?}", windows);
    for win in windows {
        let mut ele = win.split_whitespace();
        let mut typ = ele.next().unwrap();
        println!("{:#?}", typ);
    }

    file.seek(SeekFrom::Start(0)).unwrap();
    let bars = bar_colors(&file);
    println!("{:#?}", bars);

    println!("{:#?}", hash);
    Ok("".to_owned())
}

fn meta() -> HashMap<&'static str,&'static str> {
    let mut meta = HashMap::new();
    meta.insert("description", "Theme created with i3themes. https://github.com/lopukhov/i3themes");
    meta
}

fn theme_vars(file: &File) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    let lines = BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.len() > 3 && &l[0..3] == "set")
                .filter(|l| l.split_whitespace().last().unwrap_or("None").starts_with('#'))
                .map(|l| l.replace("set ", ""))
                .map(|l| l.replace("$", ""))
                .map(|l| l.replace("i3themes-", ""))
                .map(|l| l.trim().to_owned());
    for l in lines {
        let mut var = l.split_whitespace();
        let key = var.next().unwrap();
        let val = var.next().unwrap();
        vars.insert(key.to_owned(), val.to_owned());
    }
    vars
}

fn window_colors(file: &File) -> Vec<String> {
    BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.len() > 6 && &l[0..6] == "client")
                .map(|l| l.replace("client.", ""))
                .map(|l| l.replace("$", ""))
                .map(|l| l.replace("i3themes-", ""))
                .map(|l| l.trim().to_owned())
                .collect::<Vec<String>>()
}

fn bar_colors(file: &File) -> Vec<String> {
    BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.contains("_workspace") || l.contains("background") || l.contains("statusline") || l.contains("separator"))
                .filter(|l| l.len() > 6 && &l[0..6] != "client")
                .map(|l| l.replace("$", ""))
                .map(|l| l.replace("i3themes-", ""))
                .map(|l| l.trim().to_owned())
                .filter(|l| !l.starts_with('#'))
                .collect::<Vec<String>>()
}
