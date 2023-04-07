mod beautify;
mod cli;
mod config;
mod entry;
mod errors;
mod list;

use clap::Parser;
use colored::Colorize;
use list::create_list;
use std::{env, path::PathBuf, process};

use crate::{cli::ArgsSteroids, errors::BetterLsError};

fn main() {
    #[cfg(feature = "find_project_root")]
    {
        println!(
            "{}",
            "Using find_project_root feature".yellow().bold().italic()
        );
        println!(
            "{}",
            "> This should only be enabled in dev".yellow().italic()
        );
    }

    let args = cli::Args::parse();
    let paths: Vec<PathBuf>;
    let mut exit_code = 0;

    if args.paths.is_empty() {
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(err) => {
                let msg = format!("Looks like you don't have access to the current directory 😱");
                eprintln!("{}", msg.red());
                eprintln!("{}", err);
                process::exit(err.raw_os_error().unwrap_or(1));
            }
        };
        paths = vec![current_dir];
    } else {
        paths = args
            .paths
            .iter()
            .filter_map(|path_arg| {
                let path = PathBuf::from(path_arg);
                if path.exists() {
                    Some(path)
                } else {
                    exit_code = 1;
                    let msg = format!("Path doesn't exist: {}", path.display());
                    eprintln!("{}", msg.red());
                    None
                }
            })
            .collect();
    }

    println!();

    let colors = args.get_theme();
    paths.iter().for_each(|path| {
        let full_path = match path.canonicalize() {
            Ok(path) => path,
            Err(_) => path.clone(),
        };
        if full_path.is_dir() {
            if args.paths.len() > 1 {
                println!("{}:", path.display());
            }

            match create_list(full_path, &args) {
                Ok(list) => {
                    if list.size() > 0 {
                        list.print(&colors)
                    } else {
                        println!("{}", "This directory is empty.".yellow());
                    }
                }
                Err(err) => match &err {
                    // This is the one stop for errors as we show it per path listing.
                    BetterLsError::Unknown(unknown_err) => {
                        exit_code = 2;
                        eprintln!("{}", unknown_err);
                    }
                    BetterLsError::Unauthorized(_) => {
                        exit_code = 1;
                        eprintln!("{}", err);
                    }
                    BetterLsError::NotFound(_) => {
                        exit_code = 1;
                        eprintln!("{}", err);
                    }
                },
            }
            println!();
        } else {
            // TODO: this is also valid, implement later?
            // Maybe filter first to group file paths first before listing paths?
        }
    });
    process::exit(exit_code);
}
