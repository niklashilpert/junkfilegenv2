use std::{fs::{self, File, OpenOptions}, io, path::{self, Path}};

pub fn generate_file(pathStr: &str, size: u64) {
    match open_file(pathStr) {
        Ok(file) => {
            write_content(file, size);
        },
        Err(e) => {
            let formatted_kind = e.kind().to_string().to_uppercase();
            println!("[{}] An error occured whilst trying to open or create file '{}'.", formatted_kind, pathStr);
        },
    };
}

fn open_file(path_string: &str) -> io::Result<File> {
    let path = Path::new(path_string);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    return OpenOptions::new()
        .write(true)
        .create(true)
        .open(path);
}

fn write_content(file: File, size: u64) {
    println!("Writing random bytes into file...")
}