extern crate serde;

use std::io;
use std::fs::File;
use std::collections::HashMap;

use self::serde::{Serialize, Deserialize};

/// Struct to hold the information of some i3-wm theme. 
///
/// * `meta` - Optional information about the theme.
/// * `colors` - Optional maping of color name to values.
/// * `window_colors` - Window theme colors.
/// * `bar_colors` - Bar theme colors.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub meta: Option<HashMap<String, String>>,
    pub colors: Option<HashMap<String, String>>,
    pub window_colors: WinColors,
    pub bar_colors: BarColors,
}

impl Theme {
    /// Construct the colors in a way that can be added
    /// to the configuration file.
    ///
    pub fn colors(&self) -> Option<String> {
        if let Some(ref colors) = self.colors {
            let mut result = format!("{:#^100}\n\n", " i3themes variables ");
            colors.iter()
                  .map(|(k,v)| format!("set {0: <25} {1: <10}\n", "i3themes-".to_owned() + &k, v))
                  .for_each(|l| result.push_str(&l));
            Some(result)
        } else {
            None
        }
    }
}

/// Struct to hold the theme colors for the windows.
///
/// * `background` - Optional background color.
/// * `focused` - Color for focused window.
/// * `focused_inactive` - Color for inactive window.
/// * `unfocused` - Color for unfocused window.
/// * `urgent` - Color for urgent window.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WinColors {
    background: Option<String>,
    focused: HashMap<String, String>,
    focused_inactive: HashMap<String, String>,
    unfocused: HashMap<String, String>,
    urgent: HashMap<String, String>,
}

impl WinColors {
    /// Construct the colors in a way that can be added
    /// to the configuration file.
    ///
    pub fn colors(&self) -> String {
        let mut result = format!("{:#^100}\n\n", " i3themes window configuration ");
        if let Some(ref background) = self.background {
            result.push_str(&format!("client.background {}\n", background))
        }
        result.push_str(&self.format(&self.focused, "focused"));
        result.push_str(&self.format(&self.focused_inactive, "focused_inactive"));
        result.push_str(&self.format(&self.unfocused, "unfocused"));
        result.push_str(&self.format(&self.urgent, "urgent"));
        result
    }

    fn format(&self, state: &HashMap<String, String>, title: &str) -> String {
        format!("client.{0: <20} {1: <25} {2: <25} {3: <25} {4: <25}\n", title, state["border"], state["background"], state["text"], state["indicator"])
    }
}

/// Struct to hold the theme colors for the bar.
///
/// * `separator` - Separator color.
/// * `background` - Background color.
/// * `statusline` - Status line color.
/// * `focused_workspace` - Color for focused workspace.
/// * `active_workspace` - Color for active workspace.
/// * `inactive_workspace` - Color for inactive workspace.
/// * `urgent_workspace` - Color for urgent workspace.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BarColors {
    separator: String,
    background: String,
    statusline: String,
    focused_workspace: HashMap<String, String>,
    active_workspace: HashMap<String, String>,
    inactive_workspace: HashMap<String, String>,
    urgent_workspace: HashMap<String, String>,
}

impl BarColors {
    /// Construct the colors in a way that can be added
    /// to the configuration file.
    ///
    pub fn colors(&self) -> String {
        let mut result = "\tcolors {\n".to_owned();
        result.push_str(&format!("\t\t{} {}\n", "separator", &self.separator));
        result.push_str(&format!("\t\t{} {}\n", "background", &self.background));
        result.push_str(&format!("\t\t{} {}\n", "statusline", &self.statusline));
        result.push_str(&self.format(&self.focused_workspace, "focused_workspace"));
        result.push_str(&self.format(&self.active_workspace, "active_workspace"));
        result.push_str(&self.format(&self.inactive_workspace, "inactive_workspace"));
        result.push_str(&self.format(&self.urgent_workspace, "urgent_workspace"));
        result + "\t}\n"
    }

    fn format(&self, state: &HashMap<String, String>, title: &str) -> String {
        format!("\t\t{0: <20} {1: <15} {2: <15} {3: <15} \n", title, state["border"], state["background"], state["text"])
    }
}

/// Possible error when dealing with a theme.
#[derive(Debug)]
pub enum ThemeError {
    IoError(io::Error),
    YamlError(serde_yaml::Error),
}

impl From<io::Error> for ThemeError {
    fn from(err: io::Error) -> Self {
        ThemeError::IoError(err)
    }
}

impl From<serde_yaml::Error> for ThemeError {
    fn from(err: serde_yaml::Error) -> Self {
        ThemeError::YamlError(err)
    }
}

/// Load theme from file.
///
/// * `path` - Path where the theme file is located.
///
pub fn load(path: &str) -> Result<Theme, ThemeError> {
    let file = File::open(path)?;
    let mut th: Theme = serde_yaml::from_reader(&file)?;

    match th.window_colors.background {
        Some(ref mut bck) if !bck.starts_with('#') => bck.insert_str(0, "$i3themes-"),
        _ => (),
    };
    prefix(&mut th.window_colors.focused);
    prefix(&mut th.window_colors.focused_inactive);
    prefix(&mut th.window_colors.unfocused);
    prefix(&mut th.window_colors.urgent);

    th.bar_colors.separator.insert_str(0, "$i3themes-");
    th.bar_colors.background.insert_str(0, "$i3themes-");
    th.bar_colors.statusline.insert_str(0, "$i3themes-");
    prefix(&mut th.bar_colors.focused_workspace);
    prefix(&mut th.bar_colors.active_workspace);
    prefix(&mut th.bar_colors.inactive_workspace);
    prefix(&mut th.bar_colors.urgent_workspace);

    Ok(th)
}

fn prefix(map: &mut HashMap<String, String>) {
    map.iter_mut()
        .filter(|(_k,v)| !v.starts_with('#'))
        .for_each(|(_k,v)| {
            v.insert_str(0, "$i3themes-")
        });
}
