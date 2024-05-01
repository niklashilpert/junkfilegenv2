mod generator;
use clap::Parser;

// Command line argument parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    size: usize,
}


fn main() {
    let args = Args::parse();
    generator::generate_file(&args.path, args.size)
}
