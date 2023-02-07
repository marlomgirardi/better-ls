use std::fs::Metadata;
use std::io;
use std::path::PathBuf;

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
