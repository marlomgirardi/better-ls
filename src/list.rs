use std::fs::Metadata;
use std::io;
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
impl Entry {
    pub fn new(name: String, path: PathBuf, metadata: Metadata) -> Entry {
        Entry {
            name,
            path,
            metadata,
        }
    }

    pub fn from(entry: std::fs::DirEntry) -> io::Result<Entry> {
        let name = entry.file_name().into_string().unwrap();
        let path = entry.path();
        let metadata = entry.metadata()?;
        Ok(Entry::new(name, path, metadata))
    }
}
