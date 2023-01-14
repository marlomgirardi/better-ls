use chrono::prelude::{DateTime, Local};
use core::str::from_utf8_unchecked;
use libc;
use std::ffi::CStr;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::time::SystemTime;

pub fn list_entries(path: &PathBuf) {
    // Get the contents of the directory
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // Iterate over the entries and print their names and attributes
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        let path = entry.path();
        let name = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        let permissions = metadata.mode();
        let link_count = metadata.nlink();
        let size = metadata.len();
        let modified = metadata.modified().unwrap();
        let owner = get_username(metadata.uid()).unwrap();
        let group = get_groupname(metadata.gid()).unwrap();

        println!(
            "{:>10} {:>5} {:>10} {:>10} {:>10} {:>10} {:>3}  {}",
            format_permissions(permissions),
            link_count,
            owner,
            group,
            size,
            format_date(modified),
            get_icon_from_metadata(&metadata, &name.to_string_lossy()),
            name.to_string_lossy() + (if metadata.is_dir() { "/" } else { "" }),
        );
    }
}

fn format_permissions(permissions: u32) -> String {
    let mut output = String::new();

    // user permissions
    output.push(if permissions & 0o400 != 0 { 'r' } else { '-' });
    output.push(if permissions & 0o200 != 0 { 'w' } else { '-' });
    output.push(if permissions & 0o100 != 0 { 'x' } else { '-' });

    // group permissions
    output.push(if permissions & 0o040 != 0 { 'r' } else { '-' });
    output.push(if permissions & 0o020 != 0 { 'w' } else { '-' });
    output.push(if permissions & 0o010 != 0 { 'x' } else { '-' });

    // others permissions
    output.push(if permissions & 0o004 != 0 { 'r' } else { '-' });
    output.push(if permissions & 0o002 != 0 { 'w' } else { '-' });
    output.push(if permissions & 0o001 != 0 { 'x' } else { '-' });

    output
}

fn format_date(date: SystemTime) -> String {
    let datetime: DateTime<Local> = date.into();
    let formatted = datetime.format("%a %b %e %T %Y").to_string();
    formatted
}

fn get_username(uid: u32) -> io::Result<String> {
    let username = unsafe {
        let pw = libc::getpwuid(uid);
        let pw_str = CStr::from_ptr((*pw).pw_name);
        from_utf8_unchecked(pw_str.to_bytes())
    };

    if username.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not get username id {}", uid),
        ))?
    }

    Ok(username.to_string())
}

fn get_groupname(gid: u32) -> io::Result<String> {
    let groupname = unsafe {
        let gr = libc::getgrgid(gid);
        let gr_str = CStr::from_ptr((*gr).gr_name);
        from_utf8_unchecked(gr_str.to_bytes())
    };

    if groupname.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not get groupname id {}", gid),
        ))?
    }

    Ok(groupname.to_string())
}

// TODO: better way to do this?
// https://www.nerdfonts.com/cheat-sheet
fn get_icon(file_type: &str) -> String {
    match file_type {
        // directories
        "directory" => "",
        "vscode" => "",
        "symlink" => "",

        // Programming languages
        "c" => "ﭰ",
        "cpp" => "ﭱ",
        "go-lang" => "",
        "java" => "",
        "swift" => "ﯣ",
        "php" => "",
        "rust" => "",
        "python" => "",
        "javascript" => "",
        "typescript" => "",
        "html" => "",
        "css" => "",
        "json" => "",
        "yaml" => "",
        "toml" => "",
        "xml" => "",
        "svg" => "",
        "lock" => "",
        "ruby" => "",

        "test" => "",

        "compressed" => "",

        // documents
        "pdf" => "",
        "doc" => "",
        "xls" => "",

        // text
        "text" => "",
        "markdown" => "",

        // images
        "image" => "",

        // audio
        "audio" => "",

        // video
        "video" => "",

        // git
        "git" => "",
        "npm" => "",

        // OS exclusive files
        "apple" => "",
        "windows" => "",

        _ => "",
    }
    .to_string()
}

fn get_icon_from_metadata(metadata: &fs::Metadata, base_file_name: &str) -> String {
    let file_name = base_file_name.to_lowercase();

    get_icon(if metadata.is_dir() {
        match file_name {
            // Git
            _ if file_name == ".git" => "git",
            _ if file_name == ".vscode" => "vscode",
            _ if file_name == ".gem" => "ruby",
            _ if file_name == ".yarn" => "ruby",

            _ => "directory",
        }
    } else if metadata.is_file() {
        match file_name {
            _ if file_name == "package-lock.json" => "lock",
            _ if file_name == "yarn.lock" => "lock",
            _ if file_name == "pnpm-lock.yaml" => "lock",
            _ if file_name == ".gitignore" => "git",

            // OS exclusive files
            _ if file_name == ".ds_store" => "apple",
            _ if file_name == "thumbs.db" => "windows",
            _ if file_name == "desktop.ini" => "windows",

            // Test files
            _ if file_name.ends_with(".spec.js") => "test",
            _ if file_name.ends_with(".spec.ts") => "test",
            _ if file_name.ends_with(".test.js") => "test",
            _ if file_name.ends_with(".test.ts") => "test",

            // Programming Languages
            _ if file_name.ends_with(".rs") => "rust",
            _ if file_name.ends_with(".py") => "python",
            _ if file_name.ends_with(".js") => "javascript",
            _ if file_name.ends_with(".ts") => "typescript",
            _ if file_name.ends_with(".html") => "html",
            _ if file_name.ends_with(".css") => "css",
            _ if file_name.ends_with(".json") => "json",
            _ if file_name.ends_with(".toml") => "toml",
            _ if file_name.ends_with(".sh") => "shell",
            _ if file_name.ends_with(".c") => "c",
            _ if file_name.ends_with(".cpp") => "cpp",
            _ if file_name.ends_with(".h") => "h",
            _ if file_name.ends_with(".hpp") => "hpp",
            _ if file_name.ends_with(".java") => "java",
            _ if file_name.ends_with(".go") => "go-lang",
            _ if file_name.ends_with(".dart") => "dart",
            _ if file_name.ends_with(".kt") => "kotlin",
            _ if file_name.ends_with(".swift") => "swift",
            _ if file_name.ends_with(".php") => "php",
            _ if file_name.ends_with(".rb") => "ruby",
            _ if file_name.ends_with(".lua") => "lua",
            _ if file_name.ends_with(".sql") => "sql",
            _ if file_name.ends_with(".yaml") => "yaml",
            _ if file_name.ends_with(".yml") => "yaml",
            _ if file_name.ends_with(".lock") => "lock",

            // Compressed
            _ if file_name.ends_with(".zip") => "compressed",
            _ if file_name.ends_with(".tar") => "compressed",
            _ if file_name.ends_with(".gz") => "compressed",
            _ if file_name.ends_with(".xz") => "compressed",
            _ if file_name.ends_with(".bz2") => "compressed",
            _ if file_name.ends_with(".7z") => "compressed",
            _ if file_name.ends_with(".rar") => "compressed",

            // Documents
            _ if file_name.ends_with(".pdf") => "pdf",
            _ if file_name.ends_with(".doc") => "doc",
            _ if file_name.ends_with(".docx") => "doc",
            _ if file_name.ends_with(".xls") => "xls",
            _ if file_name.ends_with(".xlsx") => "xls",

            // text
            _ if file_name.ends_with(".txt") => "text",
            _ if file_name.ends_with(".log") => "text",
            _ if file_name.ends_with(".md") => "markdown",

            // images
            _ if file_name.ends_with(".png") => "image",
            _ if file_name.ends_with(".jpg") => "image",
            _ if file_name.ends_with(".jpeg") => "image",
            _ if file_name.ends_with(".gif") => "image",

            // audio
            _ if file_name.ends_with(".mp3") => "audio",
            _ if file_name.ends_with(".wav") => "audio",
            _ if file_name.ends_with(".ogg") => "audio",

            // video
            _ if file_name.ends_with(".mp4") => "video",
            _ if file_name.ends_with(".mkv") => "video",
            _ if file_name.ends_with(".webm") => "video",

            _ => "",
        }
    } else if metadata.is_symlink() {
        "symlink"
    } else {
        "unknown"
    })
}
