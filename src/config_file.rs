
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

pub fn bar_block(path: &str) -> Result<(), Box<Error>> {
    let mut bar = false;
    let file = File::open(path)?;
    let file = BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| l.len() > 0);

    for l in file {
        if l.len() > 3 && &l[0..3] == "bar" {
            bar = true;
        }
        if bar {
            println!("{}", l);
            if &l[0..1] == "}" {
                bar = false
            }
        }
    }

    Ok(())
}
