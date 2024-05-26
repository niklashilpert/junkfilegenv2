mod generator;
mod content;

use clap::Parser;

// Command line argument parser
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
    let args = Args::parse();
    generator::generate_file(args)
}
