use colored::*;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::cli::Args;
use crate::config;
use crate::config::DEFAULT_ICON_FILE;
use crate::exit_codes;
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
    for entry in entries {
        let metadata = entry.metadata;

        // TODO: break line based on terminal width
        print!("      {}", format_name(&entry.name, &metadata));
    }
    println!();
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
