use std::{cmp::min, fs::{self, File, OpenOptions}, io::{self, Write}, path::Path};

use crate::content::{PrintableCharProvider, BinaryProvider, ContentProvider};

pub fn generate_file(path_str: &str, size: usize, overwrite: bool, limit_charset: bool) {
    match open_file(path_str, overwrite) {
        Ok(file) => {
            if limit_charset {
                write_content(file, size, &mut PrintableCharProvider::new());
            } else {
                write_content(file, size, &mut BinaryProvider::new());
            }
        },
        Err(e) => {
            match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    println!("Aborting");
                },
                _ => {
                    let formatted_kind = e.kind().to_string().to_uppercase();
                    println!("[{}] An error occured whilst trying to open or create file '{}'.", formatted_kind, path_str);
                },
            }
        },
    };
}

fn open_file(path_str: &str, overwrite: bool) -> io::Result<File> {
    let path = Path::new(path_str);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.is_file() && !overwrite {
        let already_exists_message = "The file you are trying to create already exists.\nDo you want to overwrite it? [y/N]: ";
        let input = read_user_input(&already_exists_message).to_lowercase();

        if !&["y", "j"].contains(&input.as_str()) {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
    }

    return OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);
}

fn read_user_input(input_string: &str) -> String {
        print!("{}", input_string);
        _ = io::stdout().flush();

        let mut buf = String::new();
        _ = io::stdin().read_line(&mut buf);
        let trimmed_buf = buf.trim().to_string();
        return trimmed_buf;
}

fn write_content(mut file: File, size: usize, content_provider: &mut impl ContentProvider) {
    println!("Generating file...");
    let mut bytes_left = size;
    let buffer_size = 1024;

    while bytes_left > 0 {
        let batch_size = min(bytes_left, buffer_size);
        let mut buf = vec![0; batch_size];

        _ = content_provider.fill_buf(&mut buf);
        _ = file.write(&buf);

        bytes_left -= batch_size;

        print!("\rProgress: {}%", (size-bytes_left) * 100 / size);
    }
    println!("\nAll done!");
}
