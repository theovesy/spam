mod spam;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "spam")]
#[command(author, version, about, bin_name = "spam")]
struct SpamCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Make {
        #[arg(short, long)]
        input_filename: String,

        #[arg(short, long)]
        output_filename: String,

        #[arg(short, long, default_value_t = 1)]
        resize_factor: u32,
    },
    #[command(arg_required_else_help = true)]
    Template {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        width: u32,

        #[arg(short='H', long)]
        height: u32,
    },
}

fn main() {
    let args = SpamCli::parse();

    match args.command { 
        Commands::Make { 
            input_filename,
            output_filename,
            resize_factor
        } => {
            spam::process_file(&input_filename, 
                                &output_filename, 
                                resize_factor);
        }
        Commands::Template {
            filename,
            width,
            height,
        } => {
            spam::create_template(&filename, width, height);
        }
    } 
}
