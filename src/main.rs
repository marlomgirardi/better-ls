mod cli;
mod config;
mod entry;
mod errors;
mod list;

use clap::Parser;
use std::path::PathBuf;

fn main() {
    let args = cli::Args::parse();
    let mut paths: Vec<PathBuf> = args.paths.iter().map(|s| s.into()).collect();

    if paths.is_empty() {
        paths.push(cli::get_current_dir());
    }

    // TODO: handle multiple paths
    entry::list_entries(&paths[0], &args);
}
