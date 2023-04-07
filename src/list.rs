use std::{os::unix::prelude::MetadataExt, path::PathBuf};

use crate::{
    beautify::{format_date, format_permissions, get_group, get_owner},
    cli::{Args, ArgsSteroids},
    config::ColorScheme,
    entry::{get_filtered_entries, Entry},
    errors::BetterLsError,
};

pub trait List {
    fn print(&self, colors: &ColorScheme);
}

/// An inline list of entries, just icons and names.
#[derive(Debug)]
pub struct InlineList {
    entries: Vec<Entry>,
}

impl InlineList {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self { entries }
    }
}

impl List for InlineList {
    fn print(&self, _: &ColorScheme) {
        let list = self
            .entries
            .iter()
            .map(|entry| entry.display())
            .collect::<Vec<_>>();

        println!("{}", list.join("     "));
    }
}

/// A detailed list of entries being one per line, with icons, names, sizes, permissions, etc.
#[derive(Debug)]
pub struct DetailedList {
    options: DetailedListOptions,
    entries: Vec<Entry>,
}

impl DetailedList {
    pub fn new(entries: Vec<Entry>, options: DetailedListOptions) -> Self {
        Self { entries, options }
    }
}

#[derive(Debug)]
pub struct DetailedListOptions {
    pub permissions: bool,
    pub link_count: bool,
    pub owner: bool,
    pub group: bool,
    pub size: bool,
    pub modified_date: bool,
}

impl Default for DetailedListOptions {
    fn default() -> Self {
        DetailedListOptions {
            permissions: true,
            link_count: true,
            owner: true,
            group: true,
            size: true,
            modified_date: true,
        }
    }
}

impl List for DetailedList {
    fn print(&self, colors: &ColorScheme) {
        for entry in &self.entries {
            let mut line: Vec<String> = Vec::new();

            if self.options.permissions {
                let permissions = entry.metadata.mode();
                line.push(format_permissions(permissions, colors));
            }

            if self.options.link_count {
                let link_count = entry.metadata.nlink();
                line.push(link_count.to_string());
            }

            if self.options.owner {
                let owner = get_owner(entry.metadata.uid()).unwrap();
                line.push(owner);
            }

            if self.options.group {
                let group = get_group(entry.metadata.gid()).unwrap();
                line.push(group);
            }

            if self.options.size {
                let size = entry.metadata.len();
                line.push(size.to_string());
            }

            if self.options.modified_date {
                let modified = entry.metadata.modified().unwrap();
                line.push(format_date(modified));
            }

            line.push(entry.display());

            // TODO: Find a better way than using vectors. cli_table?
            println!("{}", line.join("\t"));
        }
    }
}

pub fn create_list(path: PathBuf, args: &Args) -> Result<Box<dyn List>, BetterLsError> {
    let mut entries = get_filtered_entries(&path, &args)?;

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    if args.is_long_listing() {
        let options = DetailedListOptions {
            group: args.show_group(),
            owner: !args.long_listing_no_owner,
            ..Default::default()
        };
        Ok(Box::new(DetailedList::new(entries, options)))
    } else {
        Ok(Box::new(InlineList::new(entries)))
    }
}
