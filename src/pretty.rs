
use std::fs;
use std::error::Error;

/// Obtain bar colors from the theme file in a way that
/// can be added to the configuration file.
///
/// * `theme` - Yaml object with the theme
///
pub fn bar_theme(theme: &Yaml) -> String {
    let mut result = String::new();
    let bar_types = ["focused_workspace", "active_workspace", "inactive_workspace", "urgent_workspace"];

    result.push_str("\tcolors {\n");
    result.push_str(&bglobal_colors(theme));

    for e in &bar_types {
        let color = bstate_colors(theme, e).unwrap_or("".to_string());
        result.push_str(&color);
    }

    result.push_str("\t}\n");
    result
}

fn bglobal_colors(theme: &Yaml) -> String {
    let mut colors = "".to_string();
    let bar_elements = ["separator", "background", "statusline"];
    for e in &bar_elements {
        match get_ecolor(theme, "bar_colors", e, "") {
            Some(c) => colors.push_str(&format!("\t\t{} {}\n", e, c)),
            None => continue,
        }
    }
    colors
}

fn bstate_colors(theme: &Yaml, state: &str) -> Option<String> {
    let bar_elements = ["border", "background", "text"];
    let mut elem_colors = [String::new(), String::new(), String::new()];

    for n in 0..3 {
        elem_colors[n] = get_ecolor(theme, "bar_colors", state, &bar_elements[n])?;
    }

    let colors = format!("\t\t{0: <20} {1: <15} {2: <15} {3: <15} \n", state, &elem_colors[0], &elem_colors[1], &elem_colors[2]);
    Some(colors)
}
