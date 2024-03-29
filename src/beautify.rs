use std::{ffi::CStr, fs::Metadata, io, time::SystemTime};

use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};

use crate::config::{self, ColorScheme};

pub fn get_icon_from_metadata(metadata: &Metadata, file_name: &String) -> String {
    if metadata.is_dir() {
        get_directory_icon(file_name)
    } else if metadata.is_file() {
        get_file_icon(file_name)
    } else if metadata.is_symlink() {
        get_file_icon(&"symlink".to_string())
    } else {
        get_file_icon(&"unknown".to_string())
    }
}

// FIXME? There are a few unhandled errors here.
fn get_directory_icon(dir: &str) -> String {
    let mut icon = config::DEFAULT_DIR_ICON;

    let folders = config::get_folder_icons();
    let dir = dir.to_lowercase();

    if folders.icons.contains_key(&dir) {
        icon = folders.icons.get(&dir).unwrap().as_str().unwrap();
    } else {
        let alias = folders.aliases.get(&dir);
        if let Some(key) = alias {
            icon = folders.icons.get(key).unwrap().as_str().unwrap();
        }
    }

    icon.to_string()
}

// FIXME? There are a few unhandled errors here.
fn get_file_icon(file: &String) -> String {
    let mut icon = config::DEFAULT_FILE_ICON;
    let file_icons = config::get_file_icons();
    let ext = file.split('.').last().unwrap().to_lowercase();

    if file_icons.icons.contains_key(file) {
        icon = file_icons.icons.get(file).unwrap().as_str().unwrap();
        // TODO: how to handle file.spec.ts as example?
    } else if file_icons.icons.contains_key(&ext) {
        icon = file_icons.icons.get(ext).unwrap().as_str().unwrap();
    } else {
        let mut alias = file_icons.aliases.get(file);
        if alias.is_none() {
            alias = file_icons.aliases.get(ext);
        }

        if let Some(key) = alias {
            icon = file_icons.icons.get(key).unwrap().as_str().unwrap();
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
    let pw = unsafe { libc::getpwuid(uid) };

    if pw.is_null() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("uid reference for {} is null", uid),
        ))?
    }

    let pw_str = unsafe { CStr::from_ptr((*pw).pw_name) };
    let username = pw_str.to_string_lossy().to_string();

    if username.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not get username id {}", uid),
        ))?
    }

    Ok(username)
}

pub fn get_group(gid: u32) -> io::Result<String> {
    let gr = unsafe { libc::getgrgid(gid) };

    if gr.is_null() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Group reference for {} is null", gid),
        ))?
    }

    let gr_str = unsafe { CStr::from_ptr((*gr).gr_name) };
    let groupname = gr_str.to_string_lossy().to_string();

    if groupname.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not get groupname id {}", gid),
        ))?
    }

    Ok(groupname)
}

pub fn format_date(date: SystemTime) -> String {
    let datetime: DateTime<Local> = date.into();
    let formatted = datetime.format("%a %b %e %T %Y").to_string();
    formatted
}

#[cfg(test)]
mod test {
    use crate::config::Rgb;

    use super::*;

    fn get_mocked_colors() -> ColorScheme {
        ColorScheme {
            dir: [255, 255, 255],
            recognized_file: [255, 255, 255],
            unrecognized_file: [255, 255, 255],
            executable_file: [255, 255, 255],
            read: [255, 255, 255],
            write: [255, 255, 255],
            exec: [255, 255, 255],
            no_access: [255, 255, 255],
        }
    }

    fn add_true_color(s: &str, color: Rgb) -> ColoredString {
        s.truecolor(color[0], color[1], color[2])
    }

    // #[test]
    // fn test_get_icon_from_metadata() {
    //     // TODO: Still need to understand a bit more in depth how mock works and the standards for it.
    //     let colors = get_mocked_colors();
    //     let metadata = Metadata::; ???
    //     let icon = get_icon_from_metadata(&metadata, &colors);

    //     assert_eq!(icon, add_true_color("?", colors.unrecognized_file));
    // }

    #[test]
    fn test_format_permissions() {
        let colors = get_mocked_colors();
        let formatted = format_permissions(0o777, &colors);

        let expected = format!(
            "{}{}{}{}{}{}{}{}{}",
            add_true_color("r", colors.read),
            add_true_color("w", colors.write),
            add_true_color("x", colors.exec),
            add_true_color("r", colors.read),
            add_true_color("w", colors.write),
            add_true_color("x", colors.exec),
            add_true_color("r", colors.read),
            add_true_color("w", colors.write),
            add_true_color("x", colors.exec),
        );
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_permissions_no_access() {
        let colors = get_mocked_colors();
        let formatted = format_permissions(0o000, &colors);

        let expected = format!(
            "{}{}{}{}{}{}{}{}{}",
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
            add_true_color("-", colors.no_access),
        );

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_get_owner() {
        let owner = get_owner(0).unwrap();
        assert_eq!(owner, "root");
    }

    #[test]
    fn test_get_owner_error() {
        let owner_err = get_owner(9999999).unwrap_err();
        assert_eq!(owner_err.to_string(), "uid reference for 9999999 is null");
    }

    #[test]
    fn test_get_group() {
        let group = get_group(0).unwrap();

        // TODO: Is this a good pattern?
        #[cfg(target_os = "macos")]
        {
            assert_eq!(group, "wheel");
        }
        #[cfg(not(target_os = "macos"))]
        {
            assert_eq!(group, "root");
        }
    }

    #[test]
    fn test_get_group_error() {
        let group_err = get_group(9999999).unwrap_err();
        assert_eq!(group_err.to_string(), "Group reference for 9999999 is null");
    }

    // TODO: Lots to learn about testing in rust, mocking and maybe packages instead of `SystemTime`?
    // #[test]
    // fn test_format_date() {
    //     let date = SystemTime::UNIX_EPOCH;
    //     let formatted = format_date(date);
    //     assert_eq!(formatted, "Thu Jan  1 01:00:00 1970");
    // }
}
