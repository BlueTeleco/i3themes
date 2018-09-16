
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

pub fn bar_block(path: &str) -> Result<(), Box<Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file).lines();
    file.filter_map(|l| l.ok())
        .for_each(|l| println!("{}", l));

    Ok(())
}
