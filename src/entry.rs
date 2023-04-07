use colored::Colorize;
use std::{
    fs::{self, Metadata},
    os::unix::prelude::PermissionsExt,
    path::PathBuf,
};

use crate::{
    beautify::get_icon_from_metadata,
    cli::{Args, ArgsSteroids},
    config::{ColorScheme, DEFAULT_FILE_ICON, RGB},
    errors::{exhaustive_io_error_mapping, BetterLsError},
};

#[derive(Debug)]
pub struct Entry {
    pub icon: String,
    pub name: String,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub color: RGB,
}

impl Entry {
    pub fn from_path(
        path: PathBuf,
        alias: Option<String>,
        colors: &ColorScheme,
    ) -> Result<Self, BetterLsError> {
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => todo!("Handle this case"),
        };
        let metadata = match path.metadata() {
            Ok(metadata) => metadata,
            Err(err) => return Err(exhaustive_io_error_mapping(err, file_name)),
        };
        let icon = get_icon_from_metadata(&metadata, &file_name);
        let name = alias.unwrap_or(file_name);

        let color = if metadata.is_dir() {
            colors.dir
        } else if (metadata.permissions().mode() & 0o100) == 0o100 {
            colors.executable_file
        } else if metadata.is_file() {
            if icon == DEFAULT_FILE_ICON {
                colors.unrecognized_file
            } else {
                colors.recognized_file
            }
        } else {
            colors.unrecognized_file
        };

        Ok(Self::new(name, path, icon, color, metadata))
    }

    pub fn from_parent_level(
        current_path: PathBuf,
        level: u8,
        colors: &ColorScheme,
    ) -> Result<Self, BetterLsError> {
        let mut parent_path: PathBuf = current_path;
        let mut name = String::from(".");

        // TODO: worth using enum + PartialEq and a simple if as we technically just need level 0 and 1?
        for _ in 1..=level {
            parent_path = parent_path.parent().unwrap_or(&parent_path).to_path_buf();
            name += ".";
        }

        Entry::from_path(parent_path, Some(name), colors)
    }

    pub fn new(name: String, path: PathBuf, icon: String, color: RGB, metadata: Metadata) -> Self {
        Self {
            name,
            path,
            metadata,
            icon,
            color,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "{}  {}{}",
            self.icon,
            self.name,
            if self.metadata.is_dir() { "/" } else { "" }
        )
        .truecolor(self.color[0], self.color[1], self.color[2])
        .to_string()
    }
}

// TODO: This in two functions looks nasty
/// Returns a vector of entries from a given path.
fn entries_from_path(path: &PathBuf, args: &Args) -> Result<Vec<Entry>, BetterLsError> {
    let theme = args.get_theme();

    let read_dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            return Err(exhaustive_io_error_mapping(
                err,
                path.to_string_lossy().to_string(),
            ));
        }
    };

    let entries = read_dir
        .map(|dir| match dir {
            Ok(dir) => Entry::from_path(dir.path(), None, theme),
            Err(err) => {
                // At this point we know it is not permission denied as we already checked that.
                return Err(BetterLsError::Unknown(err));
            }
        })
        .collect::<Result<Vec<_>, _>>();

    entries
}

/// Generic filtering of entries based on arguments.
pub fn get_filtered_entries(path: &PathBuf, args: &Args) -> Result<Vec<Entry>, BetterLsError> {
    let mut entries = entries_from_path(path, args)?;

    if args.directory {
        entries.retain(|entry| entry.metadata.is_dir());
    } else if args.files_only {
        entries.retain(|entry| entry.metadata.is_file());
    }

    if args.all {
        entries.push(Entry::from_parent_level(path.clone(), 0, args.get_theme())?);
        entries.push(Entry::from_parent_level(path.clone(), 1, args.get_theme())?);
    }

    if !args.show_dot_files() {
        entries.retain(|entry| !entry.name.starts_with("."))
    }

    Ok(entries)
}
