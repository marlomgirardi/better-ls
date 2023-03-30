use std::fs::Metadata;
use std::path::PathBuf;

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

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub metadata: Metadata,
}

impl TryFrom<std::fs::DirEntry> for Entry {
    type Error = std::io::Error;

    // TODO: this requires better error handling.
    fn try_from(entry: std::fs::DirEntry) -> Result<Self, Self::Error> {
        let name = entry.file_name().into_string().unwrap(); // use anyhow?
        let path = entry.path();
        let metadata = entry.metadata()?;
        Ok(Entry::new(name, path, metadata))
    }
}

impl Entry {
    pub fn new(name: String, path: PathBuf, metadata: Metadata) -> Entry {
        Entry {
            name,
            path,
            metadata,
        }
    }
}
