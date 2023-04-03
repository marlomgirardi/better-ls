mod beautify;
mod cli;
mod config;
mod entry;
mod errors;
mod list;

use clap::Parser;
use colored::Colorize;
use list::create_list;
use std::path::PathBuf;

use crate::cli::ArgsSteroids;

fn main() {
    #[cfg(unix)]
    {
        let args = cli::Args::parse();
        let paths: Vec<PathBuf>;

        if args.paths.is_empty() {
            paths = vec![cli::get_current_dir()];
        } else {
            paths = args
                .paths
                .iter()
                .filter_map(|path_arg| {
                    let path = PathBuf::from(path_arg);
                    if path.exists() {
                        Some(path)
                    } else {
                        // TODO: think about these errors.
                        let msg = format!("Specified path {} doesn't exist.", path.display());
                        eprintln!("{}", msg.red());
                        None
                    }
                })
                .collect();
        }

        println!("");

        let colors = args.get_theme();
        paths.iter().for_each(|path| {
            let full_path = path.canonicalize().unwrap();
            if full_path.is_dir() {
                if args.paths.len() > 1 {
                    println!("{}:", full_path.display())
                }
                create_list(full_path, &args).print(colors);
            } else {
                // TODO: this is also valid, implement later?
                // Maybe filter first to group file paths first before listing paths?
            }
        });
    }

    // TODO: learn a bit more about how the cfg attribute works
    #[cfg(not(unix))]
    {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "This program is only supported on Unix systems.",
        ))
    }
}
