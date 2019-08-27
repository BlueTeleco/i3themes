extern crate i3themes;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    if let Some(matches) = matches.subcommand_matches("change") {
        let config = matches.value_of("config").unwrap_or("default.conf");
        let output = matches.value_of("output").unwrap_or("out.def");
        let theme = matches.value_of("theme").unwrap_or("theme");
        i3themes::change(config, output, theme)
    }

    if let Some(matches) = matches.subcommand_matches("extract") {
        let config = matches.value_of("config").unwrap_or("default.conf");
        let output = matches.value_of("output").unwrap_or("out.def");
        i3themes::extract(config, output);
    }

    if let Some(_m) = matches.subcommand_matches("list") {
        if let Err(_e) = i3themes::list() {
            println!("The themes have not been installed correctly.");
        }
    }

    if let Some(matches) = matches.subcommand_matches("install") {
        let theme = matches.value_of("theme");
        println!("Installing {}", theme.unwrap());
    }
}
