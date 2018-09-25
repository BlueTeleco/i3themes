
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

pub struct ConfigFile {
    pub bars: Vec<String>,
    pub rest: String,
}

pub fn bar_block(path: &str) -> Result<ConfigFile, Box<Error>> {
    let file = File::open(path)?;
    let result = bars_in_file(&file);
    Ok(result)
}

fn bars_in_file(file: &File) -> ConfigFile {
    let mut result = ConfigFile{
        bars: Vec::new(),
        rest: String::new(),
    };

    let file = BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.len() < 6 || !(&l[0..6] == "client"));

    let mut bar = false;
    let mut block = String::new();
    for l in file {
        if l.len() > 3 && &l[0..3] == "bar" {
            bar = true;
        }
        if bar {
            block = format!("{}{}\n", &block, &l);
            if l.len() > 0 && &l[0..1] == "}" {
                bar = false;
                let line = format!("{}", block);
                result.bars.push(line);
                block = String::new();
            }
        } else {
            result.rest.push_str(&l);
            result.rest.push_str("\n");
        }
    }

    result
}
