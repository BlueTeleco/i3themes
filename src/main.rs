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
        i3themes::help(opts);
        return;
    }
    if matches.opt_present("v") {
        i3themes::version();
        return;
    }
    if matches.opt_present("l") {
        match i3themes::list() {
            Ok(_o) => (),
            Err(e) => println!("An error ocurred {}", e),
        }
        return;
    }
    if matches.opt_present("t") {
        i3themes::to_theme();
        return;
    }

    let input  = match matches.opt_str("c") {
        Some(i) => i,
        None => "/etc/i3/config".to_string(),
    };
    let output = match matches.opt_str("o") {
        Some(o) => o,
        None => "output.i3th".to_string(),
    };
    let theme = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        i3themes::help(opts);
        return;
    };

    i3themes::run(input, output, theme)
}
