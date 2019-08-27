
use super::theme::Theme;

use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

pub struct ConfigFile {
    pub bars: Vec<String>,
    pub rest: String,
}

/// Create the new configuration file adding the specified theme.
///
/// * `path`  - Path to the configuration file to modify
/// * `theme` - Theme to apply to the configuration file
///
pub fn build_config(path: &str, theme: Theme) -> Result<String, Box<Error>> {
    let file = File::open(path)?;

    let ConfigFile{bars, mut rest} = bars_in_file(&file);
    if let Some(s) = theme.colors() {
        rest.push_str(&s);
        rest.push_str("\n\n");
    }
    rest.push_str(&theme.window_colors.colors());
    rest.push_str("\n\n");
    for b in bars {
        let s = replace_colors(b, &theme.bar_colors.colors());
        rest.push_str(&s);
        rest.push_str("\n\n");
    }

    Ok(rest)
}

fn bars_in_file(file: &File) -> ConfigFile {
    let mut result = ConfigFile{
        bars: Vec::new(),
        rest: String::new(),
    };

    let file = BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.len() < 6 || !(&l[0..6] == "client"))
                .filter(|l| !l.contains("i3themes"));

    let mut bar = false;
    let mut block = String::new();
    for l in file {
        if l.len() > 3 && &l[0..3] == "bar" {
            bar = true;
        }
        if bar {
            block.push_str(&l);
            block.push_str("\n");

            if l.len() > 0 && &l[0..1] == "}" {
                let line = format!("{}", block);
                result.bars.push(line);
                block = String::new();
                bar = false;
            }
        } else {
            result.rest.push_str(&l);
            result.rest.push_str("\n");
        }
    }
    result
}

fn replace_colors(bar: String, colors: &str) -> String {
    let mut result = format!("{:#^100}\n\n", " i3themes bar configuration ");
    let mut lines = bar.lines();
    
    result.push_str(lines.next().unwrap_or(""));
    result.push_str("\n");

    let mut block = false;
    for l in lines {
        if l.len() > 6 && &l[0..7] == "\tcolors" {
            block = true;
            result.push_str(colors);
            result.push_str("\n");
        }
        if !block {
            result.push_str(&l);
            result.push_str("\n");
        } else if l.len() > 1 && &l[0..2] == "\t}" {
                block = false;
        }
    }
    result
}

/// Create the new yaml theme file from the selected
/// configuration file.
///
/// * `path`  - Path to the configuration file to modify
///
pub fn build_theme(path: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(path)?;

    // hash.insert("meta".to_yaml(), meta().to_yaml());
    // hash.insert("colors".to_yaml(), theme_vars(&file).to_yaml());

    file.seek(SeekFrom::Start(0)).unwrap();
    let windows = window_colors(&file);
    println!("{:#?}", windows);

    file.seek(SeekFrom::Start(0)).unwrap();
    let bars = bar_colors(&file);
    println!("{:#?}", bars);

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

fn window_colors(file: &File) -> Vec<Vec<String>> {
    BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.len() > 6 && &l[0..6] == "client")
                .map(|l| l.replace("client.", ""))
                .map(|l| l.replace("$", ""))
                .map(|l| l.replace("i3themes-", ""))
                .map(|l| l.trim().to_owned())
                .map(|l| l.split_whitespace().map(|w| w.to_owned()).collect::<Vec<String>>())
                .collect()
}

fn bar_colors(file: &File) -> Vec<Vec<String>> {
    BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.contains("_workspace") || l.contains("background") || l.contains("statusline") || l.contains("separator"))
                .filter(|l| l.len() > 6 && &l[0..6] != "client")
                .map(|l| l.replace("$", ""))
                .map(|l| l.replace("i3themes-", ""))
                .map(|l| l.trim().to_owned())
                .map(|l| l.split_whitespace().map(|w| w.to_owned()).collect::<Vec<String>>())
                .collect()
}
