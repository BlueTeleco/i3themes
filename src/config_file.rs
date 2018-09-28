
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

pub struct ConfigFile {
    pub bars: Vec<String>,
    pub rest: String,
}

pub fn output_file(path: &str, var_theme: Option<String>, win_theme: String, bar_theme: String) -> Result<String, Box<Error>> {
    let file = File::open(path)?;

    let ConfigFile{bars, mut rest} = bars_in_file(&file);
    if let Some(s) = var_theme {
        rest.push_str(&s);
        rest.push_str("\n\n");
    }
    rest.push_str(&win_theme);
    rest.push_str("\n\n");
    for b in bars {
        let s = replace_colors(b, &bar_theme);
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
    let padding = "#".repeat(26);
    let mut result = format!("{0} {1} {0}\n\n", padding, "i3themes bar configuration");
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

