extern crate i3themes;
extern crate getopts;

use std::env;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help",        "print this help menu");
    opts.optflag("v", "version",     "get current version");
    opts.optflag("l", "list-themes", "list available themes");
    opts.optopt("c",  "",            "input config file",     "<config-file>");
    opts.optopt("o",  "",            "output config file",    "<output-file>");
    opts.optopt("t",  "to-theme",    "creates an theme file", "<theme-file>");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(e) => {panic!(e.to_string())}
    };

    if matches.opt_present("h") {
        help(opts);
        return;
    }
    if matches.opt_present("v") {
        println!("Version: 0.1.0");
        return;
    }
    if matches.opt_present("l") {
        println!("ToDo");
        return;
    }
    if matches.opt_present("t") {
        println!("ToDo");
        return;
    }

    let input  = match matches.opt_str("c") {
        Some(i) => i,
        None => "stdin".to_string(),
    };
    let output = match matches.opt_str("o") {
        Some(o) => o,
        None => "stdout".to_string(),
    };
    let theme = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        help(opts);
        return;
    };

    i3themes::run(input, output, theme)
}

fn help(opts: Options) {
    println!("{} \n\n{}", "Usage:", opts.usage("i3themes <theme> [options]"));
}
