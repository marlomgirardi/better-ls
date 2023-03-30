use chrono::prelude::{DateTime, Local};
use colored::*;
use core::str::from_utf8_unchecked;
use std::ffi::CStr;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::cli::Args;
use crate::cli::ArgsSteroids;
use crate::config;
use crate::config::ColorScheme;
use crate::config::DEFAULT_ICON_FILE;
use crate::errors;
use crate::list::DetailedListOptions;
use crate::list::Entry;

pub fn list_entries(path: &PathBuf, args: &Args) {
    // Get the contents of the directory
    let mut entries = match get_entries(&path) {
        Ok(entries) => entries,
        Err(err) => {
            // TODO: handle error
            eprintln!("Error: {}", err);
            return;
        }
    };

    if args.directory {
        entries.retain(|entry| entry.metadata.is_dir());
    } else if args.files_only {
        entries.retain(|entry| entry.metadata.is_file());
    }

    if args.all {
        // TODO: handle error form unwrap
        entries.push(Entry {
            name: String::from("."),
            path: path.clone(),
            metadata: fs::metadata(".").unwrap(),
        });
        entries.push(Entry {
            name: String::from(".."),
            path: path.clone().parent().unwrap().to_path_buf(),
            metadata: fs::metadata("..").unwrap(),
        });
    }

    if !args.show_dot_files() {
        entries.retain(|entry| !entry.name.starts_with("."))
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    // Iterate over the entries and print their names and attributes

    // TODO: break line based on terminal width
    if args.is_long_listing() {
        let options = DetailedListOptions {
            group: args.show_group(),
            owner: !args.long_listing_no_owner,
            ..Default::default()
        };

        list_columns(entries, options, args.get_theme());
    } else {
        list_inline(entries, args.get_theme());
    }
}

fn format_permissions(permissions: u32, colors: &ColorScheme) -> String {
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

fn format_date(date: SystemTime) -> String {
    let datetime: DateTime<Local> = date.into();
    let formatted = datetime.format("%a %b %e %T %Y").to_string();
    formatted
}

fn get_username(uid: u32) -> io::Result<String> {
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

fn get_groupname(gid: u32) -> io::Result<String> {
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

fn format_name(name: &str, metadata: &fs::Metadata, colors: &ColorScheme) -> String {
    let mut file_name = String::from(name);

    let icon = get_icon_from_metadata(&metadata, &name);

    let name_color = if metadata.is_dir() {
        file_name.push('/');
        &colors.dir
    } else if metadata.is_file() {
        if icon == DEFAULT_ICON_FILE {
            &colors.unrecognized_file
        } else {
            &colors.recognized_file
        }
    } else {
        &colors.unrecognized_file
    };

    format!(
        "{}  {}",
        icon.truecolor(name_color[0], name_color[1], name_color[2]),
        file_name.truecolor(name_color[0], name_color[1], name_color[2])
    )
}

fn get_icon_from_metadata<'i>(metadata: &'i fs::Metadata, base_file_name: &'i str) -> &'i str {
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

fn get_directory_icon(dir: String) -> &'static str {
    let mut icon = "";

    let folders = config::get_folders();
    let dir = dir.to_lowercase();

    if folders.icons.contains_key(&dir) {
        icon = folders.icons.get(&dir).unwrap().as_str().unwrap();
    } else {
        let alias = folders.aliases.get(&dir);
        if alias.is_some() {
            icon = folders.icons.get(alias.unwrap()).unwrap().as_str().unwrap();
        }
    }

    icon
}

fn get_file_icon(file: String) -> &'static str {
    let mut icon = "";

    let files = config::get_files();
    let file = file.to_lowercase();

    let ext = file.split('.').last().unwrap();

    if files.icons.contains_key(&file) {
        icon = files.icons.get(&file).unwrap().as_str().unwrap();
        // TODO: how to handle file.spec.ts as example?
    } else if files.icons.contains_key(ext) {
        icon = files.icons.get(ext).unwrap().as_str().unwrap();
    } else {
        let alias = files.aliases.get(&file);
        if alias.is_some() {
            icon = files.icons.get(alias.unwrap()).unwrap().as_str().unwrap();
        }
    }

    icon
}

pub fn list_inline(entries: Vec<Entry>, colors: &ColorScheme) {
    let list = entries
        .iter()
        .map(|entry| format_name(&entry.name, &entry.metadata, colors))
        .collect::<Vec<_>>();
    println!("{}", list.join("     "));
}

pub fn list_columns(entries: Vec<Entry>, options: DetailedListOptions, colors: &ColorScheme) {
    for entry in entries {
        let mut line = Vec::new();
        let permissions = entry.metadata.mode();
        let link_count = entry.metadata.nlink();
        let size = entry.metadata.len();
        let modified = entry.metadata.modified().unwrap();
        let owner = get_username(entry.metadata.uid()).unwrap();
        let group = get_groupname(entry.metadata.gid()).unwrap();

        if options.permissions {
            line.push(format_permissions(permissions, colors));
        }

        if options.link_count {
            line.push(link_count.to_string());
        }

        if options.owner {
            line.push(owner);
        }

        if options.group {
            line.push(group);
        }

        if options.size {
            line.push(size.to_string());
        }

        if options.modified_date {
            line.push(format_date(modified));
        }
        line.push(format_name(&entry.name, &entry.metadata, colors));

        // TODO: Find a better way than using vectors. cli_table?
        println!("{}", line.join("\t"));
    }
}

pub fn get_entries(path: &PathBuf) -> io::Result<Vec<Entry>> {
    let entries = fs::read_dir(path)?
        .map(|res| {
            res.map(|e| match e.try_into() {
                Ok(entry) => entry,
                Err(err) => {
                    // TODO: handle error
                    eprintln!("Error: {}", err);
                    // if err.kind() == io::ErrorKind::PermissionDenied {
                    //     std::process::exit(errors::PERMISSION_DENIED);
                    // }
                    std::process::exit(errors::SERIOUS_TROUBLE);
                }
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}
