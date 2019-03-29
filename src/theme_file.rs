
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

pub fn output_file(path: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(path)?;

    let vars = theme_vars(&file);
    println!("{:?}", vars);

    file.seek(SeekFrom::Start(0));
    let windows = window_colors(&file);
    for s in windows {
        println!("{}", s);
    }

    file.seek(SeekFrom::Start(0));
    let bars = bar_colors(&file);
    for s in bars {
        println!("{}", s);
    }

    Ok("HOla".to_owned())
}

fn theme_vars(file: &File) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    let file = BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.len() > 3 && &l[0..3] == "set")
                .filter(|l| l.split_whitespace().last().unwrap_or("None").starts_with('#'))
                .map(|l| l.replace("set ", ""))
                .map(|l| l.replace("i3themes-", ""));

    for l in file {
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
                .map(|l| l.replace("i3themes-", ""))
                .collect::<Vec<String>>()
}

fn bar_colors(file: &File) -> Vec<String> {
    BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.contains("_workspace") || l.contains("background") || l.contains("statusline") || l.contains("separator"))
                .filter(|l| !l.contains("#"))
                .map(|l| l.trim().to_owned())
                .map(|l| l.replace("i3themes-", ""))
                .collect::<Vec<String>>()
}
