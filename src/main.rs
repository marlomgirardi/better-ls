mod cli;
mod config;
mod entry;
mod errors;
mod list;

use clap::Parser;
use std::path::PathBuf;

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
                        eprintln!("Specified path {} doesn't exist.", path.display());
                        None
                    }
                })
                .collect();
        }

        // TODO: handle multiple paths
        entry::list_entries(&paths[0], &args);
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
