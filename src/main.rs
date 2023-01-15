mod cli;
mod config;
mod entry;

use clap::Parser;
use std::path::PathBuf;

fn main() {
    let args = cli::Args::parse();
    let mut paths: Vec<PathBuf> = args.paths.iter().map(|s| s.into()).collect();

    if paths.len() == 0 {
        paths.push(cli::get_current_dir());
    }

    if paths.len() > 1 {
        // TODO: handle multiple paths
        for path in paths {
            entry::list_entries(&path);
        }
    } else {
        entry::list_entries(&paths[0]);
    }
}
