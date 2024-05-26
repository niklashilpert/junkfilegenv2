mod content;

use std::{cmp::min, fs::{self, File, OpenOptions}, io::{self, Write}, path::Path, time};

use clap::Parser;

use crate::content::{BinaryProvider, ContentProvider, PrintableCharProvider};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    size: usize,

    #[arg(short, long, default_value_t = false)]
    overwrite: bool,

    #[arg(short = 'l', long, default_value_t = false)]
    limit_charset: bool,

    #[arg(short = 'd', long, default_value_t = false)]
    always_use_default: bool
}


fn main() {
    generate_file(Args::parse());
}


pub fn generate_file(args: Args) {
    match open_file(&args.path, args.overwrite) {
        Ok(file) => {
            if args.limit_charset {
                write_content(file, args.size, &mut PrintableCharProvider::new());
            } else {
                write_content(file, args.size, &mut BinaryProvider::new(args.always_use_default));
            }
        },
        Err(e) => {
            match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    println!("Aborting.");
                },
                _ => {
                    let formatted_kind = e.kind().to_string().to_uppercase();
                    println!("[{}] An error occured whilst trying to open file '{}'.", formatted_kind, args.path);
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
    let start_time = time::Instant::now();

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

    let elapsed = start_time.elapsed().as_millis();
    let seconds = elapsed / 1000;
    let millis = elapsed - seconds * 1000;

    println!("\nAll done! ({}s and {}ms)", seconds, millis);
}

