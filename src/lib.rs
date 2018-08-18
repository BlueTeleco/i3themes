extern crate yaml_rust;

mod yaml;

use std::io;
use std::fs;
use yaml::*;
use yaml_rust::Yaml;

pub fn run(input: String, output: String, theme: String) {
    let path = format!("themes/{}", theme);
    println!("Input: {} --- Output: {} --- Theme:\n\n{} \n\n{}", input, output, window_theme(&path), bar_theme(&path));
}

pub fn list() -> io::Result<()> {
    println!("Available themes:\n");

    for entry in fs::read_dir("themes")? {
        let entry = entry?;
        let path = entry.path();
        let path = match path.to_str() {
            Some(s) => s,
            None => continue,
        };

        let theme = match load_yaml(path) {
            Ok(y) => y,
            Err(_e) => continue,
        };

        let desc = get_yaml_str(&theme[0], "meta", "description", "")
                    .unwrap_or("Description not found".to_string());

        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(_s) => continue,
        };
        println!("\t{0: <20} {1: <5} {2: <100}", name, "-->", desc);
    }
    Ok(())
}

fn window_theme(path: &str) -> String {
    let padding = "#".repeat(26);
    let mut result = format!("{0} {1} {0}\n\n", padding, "i3themes window configuration");
    let win_types = ["focused", "unfocused", "focused_inactive", "urgent"];

    let theme = load_yaml(path).unwrap_or_else(|_e| {
            println!("Error loading the theme, try again or submit a bug report");
            vec![Yaml::BadValue]
    });

    for e in &win_types {
        let color = wstate_colors(&theme[0], e).unwrap_or_else(|| {
                println!("Error loading the theme, try again or submit a bug report");
                "".to_string()
        });
        result.push_str(&color);
    }
    result + "\n" + &"#".repeat(83)
}

fn wstate_colors(theme: &Yaml, state: &str) -> Option<String> {
    let win_elements = ["border", "background", "text", "indicator"];
    let mut elem_colors = [String::new(), String::new(), String::new(), String::new()];

    for n in 0..4 {
        elem_colors[n] = get_yaml_str(theme, "window_colors", state, &win_elements[n])?;
    }

    let colors = format!("client.{0: <20} {1: <15} {2: <15} {3: <15} {4: <15}\n", state, &elem_colors[0], &elem_colors[1], &elem_colors[2], &elem_colors[3]);
    Some(colors)
}

fn bar_theme(path: &str) -> String {
    let padding = "#".repeat(26);
    let mut result = format!("{0} {1} {0}\n\n", padding, "i3themes bar configuration");
    result.push_str("\tcolor {\n");
    let bar_types = ["focused_workspace", "active_workspace", "inactive_workspace", "urgent_workspace"];

    let theme = load_yaml(path).unwrap_or_else(|_e| {
            println!("Error loading the theme, try again or submit a bug report");
            vec![Yaml::BadValue]
    });
    
    let global = bglobal_colors(&theme[0]).unwrap_or_else(|| {
            println!("Error loading the theme, try again or submit a bug report");
            "".to_string()
    });
    result.push_str(&global);

    for e in &bar_types {
        let color = bstate_colors(&theme[0], e).unwrap_or_else(|| {
                println!("Error loading the theme, try again or submit a bug report");
                "".to_string()
        });
        result.push_str(&color);
    }

    result.push_str("\t}\n");
    result + "\n" + &"#".repeat(80)
}

fn bglobal_colors(theme: &Yaml) -> Option<String> {
    let separator = get_yaml_str(theme, "bar_colors", "separator", "")?;
    let background = get_yaml_str(theme, "bar_colors", "background", "")?;
    let statusline = get_yaml_str(theme, "bar_colors", "statusline", "")?;

    let colors = format!("\t\tseparator {}\n\t\tbackground {}\n\t\tstatusline {}\n", separator, background, statusline);
    Some(colors)
}

fn bstate_colors(theme: &Yaml, state: &str) -> Option<String> {
    let win_elements = ["border", "background", "text"];
    let mut elem_colors = [String::new(), String::new(), String::new()];

    for n in 0..3 {
        elem_colors[n] = get_yaml_str(theme, "bar_colors", state, &win_elements[n])?;
    }

    let colors = format!("\t\t{0: <20} {1: <15} {2: <15} {3: <15} \n", state, &elem_colors[0], &elem_colors[1], &elem_colors[2]);
    Some(colors)
}
