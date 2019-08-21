extern crate serde;

use std::io;
use std::fs::File;
use std::collections::HashMap;

use self::serde::{Serialize, Deserialize};

/// Struct to hold the information of some i3-wm theme. 
/// * `meta` - Optional information about the theme.
/// * `colors` - Optional maping of color name to values.
/// * `window_colors` - Window theme colors.
/// * `bar_colors` - Bar theme colors.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    meta: Option<HashMap<String, String>>,
    colors: Option<HashMap<String, String>>,
    window_colors: WinColors,
    bar_colors: BarColors,
}

/// Struct to hold the theme colors for the windows.
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

/// Struct to hold the theme colors for the bar.
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
/// * `path` - Path where the theme file is located.
///
pub fn load(path: &str) -> Result<Theme, ThemeError> {
    let file = File::open(path)?;
    let th: Theme = serde_yaml::from_reader(&file)?;
    Ok(th)
}
