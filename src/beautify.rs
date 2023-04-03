use std::{ffi::CStr, fs::Metadata, io, str::from_utf8_unchecked, time::SystemTime};

use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};

use crate::config::{self, ColorScheme};

pub fn get_icon_from_metadata(metadata: &Metadata, base_file_name: &String) -> String {
    let file_name = base_file_name.to_lowercase();

    if metadata.is_dir() {
        get_directory_icon(file_name)
    } else if metadata.is_file() {
        get_file_icon(file_name)
    } else if metadata.is_symlink() {
        get_file_icon("symlink".to_string())
    } else {
        get_file_icon("unknown".to_string())
    }
}

fn get_directory_icon(dir: String) -> String {
    let mut icon = config::DEFAULT_DIR_ICON;

    let folders = config::get_folder_icons();
    let dir = dir.to_lowercase();

    if folders.icons.contains_key(&dir) {
        icon = folders.icons.get(&dir).unwrap().as_str().unwrap();
    } else {
        let alias = folders.aliases.get(&dir);
        if alias.is_some() {
            icon = folders.icons.get(alias.unwrap()).unwrap().as_str().unwrap();
        }
    }

    icon.to_string()
}

fn get_file_icon(file: String) -> String {
    let mut icon = config::DEFAULT_FILE_ICON;

    let file_icons = config::get_file_icons();
    let file = file.to_lowercase();

    let ext = file.split('.').last().unwrap();

    if file_icons.icons.contains_key(&file) {
        icon = file_icons.icons.get(&file).unwrap().as_str().unwrap();
        // TODO: how to handle file.spec.ts as example?
    } else if file_icons.icons.contains_key(ext) {
        icon = file_icons.icons.get(ext).unwrap().as_str().unwrap();
    } else {
        let alias = file_icons.aliases.get(&file);
        if alias.is_some() {
            icon = file_icons
                .icons
                .get(alias.unwrap())
                .unwrap()
                .as_str()
                .unwrap();
        }
    }

    icon.to_string()
}

pub fn format_permissions(permissions: u32, colors: &ColorScheme) -> String {
    let user = format!(
        "{}{}{}",
        colorize_read(permissions & 0o400 != 0, colors),
        colorize_write(permissions & 0o200 != 0, colors),
        colorize_exec(permissions & 0o100 != 0, colors)
    );

    let group = format!(
        "{}{}{}",
        colorize_read(permissions & 0o040 != 0, colors),
        colorize_write(permissions & 0o020 != 0, colors),
        colorize_exec(permissions & 0o010 != 0, colors)
    );

    let others = format!(
        "{}{}{}",
        colorize_read(permissions & 0o004 != 0, colors),
        colorize_write(permissions & 0o002 != 0, colors),
        colorize_exec(permissions & 0o001 != 0, colors)
    );

    format!("{}{}{}", user, group, others)
}

fn colorize_read(read: bool, colors: &ColorScheme) -> ColoredString {
    if read {
        "r".truecolor(colors.read[0], colors.read[1], colors.read[2])
    } else {
        "-".truecolor(
            colors.no_access[0],
            colors.no_access[1],
            colors.no_access[2],
        )
    }
}

fn colorize_write(write: bool, colors: &ColorScheme) -> ColoredString {
    if write {
        "w".truecolor(colors.write[0], colors.write[1], colors.write[2])
    } else {
        "-".truecolor(
            colors.no_access[0],
            colors.no_access[1],
            colors.no_access[2],
        )
    }
}

fn colorize_exec(exec: bool, colors: &ColorScheme) -> ColoredString {
    if exec {
        "x".truecolor(colors.exec[0], colors.exec[1], colors.exec[2])
    } else {
        "-".truecolor(
            colors.no_access[0],
            colors.no_access[1],
            colors.no_access[2],
        )
    }
}

pub fn get_owner(uid: u32) -> io::Result<String> {
    let username = unsafe {
        let pw = libc::getpwuid(uid);
        let pw_str = CStr::from_ptr((*pw).pw_name);
        from_utf8_unchecked(pw_str.to_bytes())
    };

    if username.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not get username id {}", uid),
        ))?
    }

    Ok(username.to_string())
}

pub fn get_group(gid: u32) -> io::Result<String> {
    let groupname = unsafe {
        let gr = libc::getgrgid(gid);
        let gr_str = CStr::from_ptr((*gr).gr_name);
        from_utf8_unchecked(gr_str.to_bytes())
    };

    if groupname.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not get groupname id {}", gid),
        ))?
    }

    Ok(groupname.to_string())
}

pub fn format_date(date: SystemTime) -> String {
    let datetime: DateTime<Local> = date.into();
    let formatted = datetime.format("%a %b %e %T %Y").to_string();
    formatted
}
