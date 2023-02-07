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
