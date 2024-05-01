use std::{cmp::min, fs::{self, File, OpenOptions}, io::{self, Read, Write}, path::Path};

pub fn generate_file(path_str: &str, size: usize) {
    match open_file(path_str) {
        Ok(file) => {
            write_content(file, size);
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

fn open_file(path_str: &str) -> io::Result<File> {
    let path = Path::new(path_str);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.is_file() {
        let already_exists_message = "The file you are trying to create already exists.\nDo you want to overwrite it? [y/N]: ";
        let input = read_user_input(&already_exists_message).to_lowercase();

        if !&["y", "j"].contains(&input.as_str()) {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
    }

    return OpenOptions::new()
        .write(true)
        .create(true)
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


fn write_content(mut file: File, size: usize) {
    let mut bytes_left = size;
    let buffer_size = 1024;
    let mut random_file: File = File::open("/dev/random").unwrap();

    while bytes_left > 0 {
        let batch_size = min(bytes_left, buffer_size);
        let mut buf = vec![0; batch_size];
        _ = random_file.read_exact(&mut buf);
        _ = file.write(&buf);

        bytes_left -= batch_size;

        print!("\rProgress: {}%", (size-bytes_left) * 100 / size);
    }
    println!("\nAll done!");
}
