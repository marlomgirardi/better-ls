use colored::Colorize;
use lazy_static::lazy_static;
use serde::Deserialize;

/// IconMapping is used to map icons and alias to files and directories.
#[derive(Debug, Deserialize)]
pub struct IconMapping {
    pub icons: serde_yaml::Mapping,
    pub aliases: serde_yaml::Mapping,
}

impl Default for IconMapping {
    fn default() -> Self {
        println!("{}", "Using default icon mapping".yellow());
        IconMapping {
            icons: serde_yaml::Mapping::new(),
            aliases: serde_yaml::Mapping::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub dark: ColorScheme,
    // pub light: ColorScheme,
}

impl Default for Colors {
    fn default() -> Self {
        println!("{}", "Using default colors".yellow());
        let white = [255, 255, 255];
        Colors {
            dark: ColorScheme {
                dir: white,
                recognized_file: white,
                unrecognized_file: white,
                executable_file: white,
                read: white,
                write: white,
                exec: white,
                no_access: white,
            },
        }
    }
}

// Rgb type vec
pub type Rgb = [u8; 3];

#[derive(Debug, Deserialize)]
pub struct ColorScheme {
    pub dir: Rgb,
    pub recognized_file: Rgb,
    pub unrecognized_file: Rgb,
    pub executable_file: Rgb,
    pub read: Rgb,
    pub write: Rgb,
    pub exec: Rgb,
    pub no_access: Rgb,
}

static COLORS_YAML: &'static str = include_str!("../config/colors.yml");
static FOLDERS_YAML: &'static str = include_str!("../config/folders.yml");
static FILES_YAML: &'static str = include_str!("../config/files.yml");

// TODO: worth leting it as yaml to add custom configuration later, or just use the struct directly?
lazy_static! {
    #[derive(Debug)]
    static ref COLORS: Colors = serde_yaml::from_str(COLORS_YAML).unwrap_or_default();
    #[derive(Debug)]
    static ref FOLDER_ICONS: IconMapping = serde_yaml::from_str(FOLDERS_YAML).unwrap_or_default();
    #[derive(Debug)]
    static ref FILE_ICONS: IconMapping = serde_yaml::from_str(FILES_YAML).unwrap_or_default();
}

pub const DEFAULT_DIR_ICON: &str = "";
pub const DEFAULT_FILE_ICON: &str = "";

pub enum Theme {
    Dark,
    Light,
}

pub fn get_colors(theme: Theme) -> &'static ColorScheme {
    let colors = &*COLORS;
    match theme {
        Theme::Dark => &colors.dark,
        // Theme::Light => &colors.light,
        _ => &colors.dark,
    }
}

pub fn get_folder_icons() -> &'static IconMapping {
    &FOLDER_ICONS
}

pub fn get_file_icons() -> &'static IconMapping {
    &FILE_ICONS
}
