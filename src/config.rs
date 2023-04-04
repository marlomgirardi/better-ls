use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

/// IconMapping is used to map icons and alias to files and directories.
#[derive(Debug, Deserialize)]
pub struct IconMapping {
    pub icons: serde_yaml::Mapping,
    pub aliases: serde_yaml::Mapping,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub dark: ColorScheme,
    // pub light: ColorScheme,
}

// RGB type vec
pub type RGB = [u8; 3];

#[derive(Debug, Deserialize)]
pub struct ColorScheme {
    pub dir: RGB,
    pub recognized_file: RGB,
    pub unrecognized_file: RGB,
    pub executable_file: RGB,
    pub read: RGB,
    pub write: RGB,
    pub exec: RGB,
    pub no_access: RGB,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref COLORS: Colors = get_config_file("config/colors.yml");
    #[derive(Debug)]
    pub static ref FOLDER_ICONS: IconMapping = get_config_file("config/folders.yml");
    #[derive(Debug)]
    pub static ref FILE_ICONS: IconMapping = get_config_file("config/files.yml");
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
    &*FOLDER_ICONS
}

pub fn get_file_icons() -> &'static IconMapping {
    &*FILE_ICONS
}

fn get_config_file<YamlType>(path: &str) -> YamlType
where
    YamlType: for<'de> Deserialize<'de>,
{
    let full_path = get_project_root_dir().join(path);
    let file = File::open(full_path).unwrap();
    let folders: YamlType = serde_yaml::from_reader(file).unwrap();
    folders
}

/// Get the root directory of the project.
/// Required mainly when running within one of the directories of the project.
fn get_project_root_dir() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir
        .ancestors()
        .find(|dir| dir.join("Cargo.toml").exists())
        .unwrap()
        .to_path_buf()
}
