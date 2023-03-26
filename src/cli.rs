use clap::Parser;
use std::{env, path};

#[derive(Parser, Debug)]
#[command(bin_name = "bls")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// do not ignore entries starting with "."
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    /// do not list implied . and ..
    #[arg(short = 'A', long, default_value_t = false)]
    pub almost_all: bool,

    /// use a long listing format
    #[arg(short, default_value_t = false)]
    pub long_listing: bool,

    /// like `-l`, but do not list owner
    #[arg(short = 'g', default_value_t = false)]
    pub long_listing_no_owner: bool,

    /// in a long listing, don't print group names
    #[arg(short = 'G', long, default_value_t = false)]
    pub no_group: bool,

    /// like -l, but do not list group information
    #[arg(short = 'o', default_value_t = false)]
    pub long_listing_no_group: bool,

    /// list directories themselves, not their contents
    #[arg(short, long, default_value_t = false)]
    pub directory: bool,

    /// list all entries in directory order
    #[arg(short, default_value_t = false)]
    pub files_only: bool,

    /// list entries by paths.
    pub paths: Vec<String>,
}

pub fn get_current_dir() -> path::PathBuf {
    match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            // TODO: handle error (no dir or no permission)
            panic!("Error: {}", e);
        }
    }
}
