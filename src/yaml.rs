extern crate yaml_rust;

use std::fs;
use yaml_rust::*;
use std::error::Error;

/// Load contents of a yaml file into an Yaml object.
///
/// * `path` - Path to yaml file
///
pub fn load_yaml(path: &str) -> Result<Vec<Yaml>, Box<Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(YamlLoader::load_from_str(&contents)?)
}

/// Get string from a yaml hierarchy.
///
/// * `first`  - First key in hierarchy
/// * `second` - Second key in hierarchy or empty str
/// * `third`  - Third key in hierarchy or empty str
///
pub fn get_yaml_str(yaml: &Yaml, first: &str, second: &str, third: &str) -> Option<String> {
    let h = yaml.as_hash()?;
    if first != "" {
        let value = h.get(&Yaml::from_str(first))?;
        match value {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::Hash(_h) => get_yaml_str(value, second, third, ""),
            _ => None,
        }
    } else {
        None
    }
}

fn get_ecolor(theme: &Yaml, color_set: &str, state: &str, element: &str) -> Option<String> {
    let mut ecolor = get_yaml_str(theme, color_set, state, element)?;
    if !ecolor.starts_with('#') {
        ecolor.insert_str(0, "$i3themes-");
    }
    Some(ecolor)
}

/// Obtain variables from the theme file in a way that
/// can be added to the configuration file.
///
/// * `theme` - Yaml object with the theme
///
pub fn theme_vars(theme: &Yaml) -> Option<String> {
    let mut result = format!("{:#^100}\n\n", " i3themes variables ");

    let colors = theme.as_hash()?
                      .get(&Yaml::from_str("colors"))?
                      .as_hash()?;

    for k in colors.keys() {
        let prefix = "i3themes-".to_owned();
        let key = k.as_str()?;
        let val = colors.get(k)?.as_str()?;
        let var = format!("set ${0: <25} {1: <10}\n", prefix + key, val);
        result.push_str(&var);
    }

    Some(result)
}

/// Obtain window colors from the theme file in a way that
/// can be added to the configuration file.
///
/// * `theme` - Yaml object with the theme
///
pub fn window_theme(theme: &Yaml) -> String {
    let mut result = format!("{:#^100}\n\n", " i3themes window configuration ");
    let win_types = ["focused", "unfocused", "focused_inactive", "urgent"];

    for e in &win_types {
        let color = wstate_colors(theme, e).unwrap_or("".to_string());
        result.push_str(&color);
    }
    result
}

fn wstate_colors(theme: &Yaml, state: &str) -> Option<String> {
    let win_elements = ["border", "background", "text", "indicator"];
    let mut elem_colors = [String::new(), String::new(), String::new(), String::new()];

    for n in 0..4 {
        elem_colors[n] = get_ecolor(theme, "window_colors", state, &win_elements[n])?;
    }

    let colors = format!("client.{0: <20} {1: <25} {2: <25} {3: <25} {4: <25}\n", state, &elem_colors[0], &elem_colors[1], &elem_colors[2], &elem_colors[3]);
    Some(colors)
}

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
