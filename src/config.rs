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

lazy_static! {
    #[derive(Debug)]
    static ref COLORS: Colors = get_config_file("config/colors.yml");
    #[derive(Debug)]
    static ref FOLDER_ICONS: IconMapping = get_config_file("config/folders.yml");
    #[derive(Debug)]
    static ref FILE_ICONS: IconMapping = get_config_file("config/files.yml");
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

// Only used for lazy_static,

fn get_config_file<YamlType>(path: &str) -> YamlType
where
    YamlType: for<'de> Deserialize<'de>,
{
    let path_buf: PathBuf;

    #[cfg(feature = "find_project_root")]
    {
        // Required mainly when running within one of the directories of the project.
        path_buf = std::env::current_dir()
            .unwrap()
            .ancestors()
            .find(|dir| dir.join("Cargo.toml").exists())
            .unwrap()
            .to_path_buf()
            .join(path);
    }

    #[cfg(not(feature = "find_project_root"))]
    {
        path_buf = PathBuf::from(path);
    }

    let file = File::open(path_buf).unwrap();
    let folders: YamlType = serde_yaml::from_reader(file).unwrap();
    folders
}
