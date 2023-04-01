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
        let mut paths: Vec<PathBuf> = args.paths.iter().map(|s| s.into()).collect();

        if paths.is_empty() {
            paths.push(cli::get_current_dir());
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
