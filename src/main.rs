mod spam;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input_filename: String,

    #[arg(short, long)]
    output_filename: String,

    #[arg(short, long, default_value_t = 1)]
    resize_factor: u32,
}

fn main() {
    let args = Args::parse();
    spam::process_file(&args.input_filename, 
        &args.output_filename, args.resize_factor);
}
