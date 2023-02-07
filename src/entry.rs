use chrono::prelude::{DateTime, Local};
use colored::*;
use core::str::from_utf8_unchecked;
use libc;
use std::ffi::CStr;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::cli::Args;
use crate::config;
use crate::config::DEFAULT_ICON_FILE;
use crate::exit_codes;
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

    if !args.all && !args.almost_all {
        filter_entries(&mut entries);
    }

    sort_entries(&mut entries);

    // Iterate over the entries and print their names and attributes

    // TODO: break line based on terminal width
    let long = args.long_listing || args.long_listing_no_group || args.long_listing_no_owner;
    if long {
        let options = DetailedListOptions {
            group: !(args.long_listing_no_group || args.no_group),
            owner: !args.long_listing_no_owner,
            ..Default::default()
        };

        list_columns(entries, options);
    } else {
        list_inline(entries);
    }
}

fn format_permissions(permissions: u32) -> String {
    let mut output = String::new();

    // user permissions
    output.push(if permissions & 0o400 != 0 { 'r' } else { '-' });
    output.push(if permissions & 0o200 != 0 { 'w' } else { '-' });
    output.push(if permissions & 0o100 != 0 { 'x' } else { '-' });

    // group permissions
    output.push(if permissions & 0o040 != 0 { 'r' } else { '-' });
    output.push(if permissions & 0o020 != 0 { 'w' } else { '-' });
    output.push(if permissions & 0o010 != 0 { 'x' } else { '-' });

    // others permissions
    output.push(if permissions & 0o004 != 0 { 'r' } else { '-' });
    output.push(if permissions & 0o002 != 0 { 'w' } else { '-' });
    output.push(if permissions & 0o001 != 0 { 'x' } else { '-' });

    output
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

fn format_name(name: &str, metadata: &fs::Metadata) -> String {
    let colors = config::get_colors(None); // TODO: get from config?
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
    } else if files.icons.contains_key(&ext) {
        icon = files.icons.get(&ext).unwrap().as_str().unwrap();
    } else {
        let alias = files.aliases.get(&file);
        if alias.is_some() {
            icon = files.icons.get(alias.unwrap()).unwrap().as_str().unwrap();
        }
    }

    icon
}

pub fn list_inline(entries: Vec<Entry>) {
    for entry in entries {
        let metadata = entry.metadata;
        print!("{} ", format_name(&entry.name, &metadata));
    }
}

pub fn list_columns(entries: Vec<Entry>, options: DetailedListOptions) {
    for entry in entries {
        let mut line = Vec::new();
        let permissions = entry.metadata.mode();
        let link_count = entry.metadata.nlink();
        let size = entry.metadata.len();
        let modified = entry.metadata.modified().unwrap();
        let owner = get_username(entry.metadata.uid()).unwrap();
        let group = get_groupname(entry.metadata.gid()).unwrap();

        if options.permissions {
            line.push(format_permissions(permissions));
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
        line.push(format_name(&entry.name, &entry.metadata));

        // TODO: Find a better way than using vectors. cli_table?
        println!("{}", line.join("\t"));
    }
}

pub fn get_entries(path: &PathBuf) -> io::Result<Vec<Entry>> {
    let entries = fs::read_dir(path)?
        .map(|res| {
            res.map(|e| match Entry::from(e) {
                Ok(entry) => entry,
                Err(err) => {
                    // TODO: handle error
                    eprintln!("Error: {}", err);
                    // if err.kind() == io::ErrorKind::PermissionDenied {
                    //     std::process::exit(exit_codes::PERMISSION_DENIED);
                    // }
                    std::process::exit(exit_codes::SERIOUS_TROUBLE);
                }
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

pub fn sort_entries(entries: &mut Vec<Entry>) -> &mut Vec<Entry> {
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    entries
}

pub fn filter_entries(entries: &mut Vec<Entry>) -> &mut Vec<Entry> {
    entries.retain(|entry| !entry.name.starts_with("."));
    entries
}
