mod size;
mod content;

use std::{cmp::min, fs::{self, File, OpenOptions}, io::{self, Write}, path::Path, time};

use clap::Parser;

use crate::content::{BinaryProvider, ContentProvider, PrintableCharProvider};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    path: Option<String>,

    #[arg(short, long)]
    size: String,

    #[arg(short = 'v', long, default_value_t = 0.2)]
    deviation_factor: f64,

    #[arg(short, long, default_value_t = false)]
    overwrite: bool,

    #[arg(short, long, default_value_t = false)]
    limit_charset: bool,

    #[arg(short = 'd', long, default_value_t = false)]
    always_use_default: bool
}


fn main() {
    let args = Args::parse();

    if args.deviation_factor < 0.0 || args.deviation_factor >= 1.0 {
        println!("Deviation factor must fulfill the following rule: 0 <= x < 1");
        return
    }

    generate_content(Args::parse())
}


pub fn generate_content(args: Args) {
    match size::from((&args.size).to_string(), args.deviation_factor) {
        Some(size) => {
            match open_output(&args) {
                Ok((out, is_console)) => {
                    if args.limit_charset {
                        write_content(out, size, is_console, &mut PrintableCharProvider::new());
                    } else {
                        write_content(out, size, is_console, &mut BinaryProvider::new(args.always_use_default));
                    }                
                },
                Err(e) => {
                    match e.kind() {
                        io::ErrorKind::InvalidInput => {
                            println!("The provided file name is empty.");
                        }
                        io::ErrorKind::AlreadyExists => {
                            println!("Aborting.");
                        },
                        io::ErrorKind::PermissionDenied => {
                            println!("You don't have permission to write to the file.");
                        }
                        _ => {
                            let formatted_kind = e.kind().to_string().to_uppercase();
                            println!("[{}] An error occured whilst trying to open the file.", formatted_kind);
                        },
                    }
                }
            }
        },
        None => {
            println!("Could not parse size string.");
        },
    }
}

fn open_output(args: &Args) -> io::Result<(Box<dyn Write>, bool)> {
    let mut out: Box<dyn Write> = Box::from(io::stdout());
    let mut is_console = true;
    if let Some(path) = &args.path {
        if path == "" {
            return Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
        let file_res = open_file(path, args.overwrite);
        match file_res {
            Ok(file) => {
                out = Box::from(file);
                is_console = false;
            },
            Err(e) => {
                return Err(e);
            },
        };
    }
    Ok((Box::from(out), is_console))
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

fn write_content(mut out: impl Write, size: usize, log: bool, content_provider: &mut impl ContentProvider) {
    if !log {
        println!("Generating file... ({} Bytes)", size);
    }
    let start_time = time::Instant::now();

    let mut bytes_left = size;
    let buffer_size = 1024;

    while bytes_left > 0 {
        let batch_size = min(bytes_left, buffer_size);
        let mut buf = vec![0; batch_size];

        _ = content_provider.fill_buf(&mut buf);
        _ = out.write(&buf);

        bytes_left -= batch_size;

        if !log {
            print!("\rProgress: {}%", (size-bytes_left) * 100 / size);
        }
    }
    _ = out.flush();
    let elapsed = start_time.elapsed().as_millis();
    let seconds = elapsed / 1000;
    let millis = elapsed - seconds * 1000;

    if !log {
        println!("\nAll done! ({}s and {}ms)", seconds, millis);
    }
}

