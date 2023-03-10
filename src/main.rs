use clap::Parser;

mod fragment;
mod split;

/// Simple CSV file splitter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// CSV file to split
    input_filepath: String,

    /// Line count of each splitted file
    line_count: usize,

    /// Output file prefix
    #[arg(short, long)]
    prefix: Option<String>,

    /// Output file directory
    #[arg(short, long)]
    output_dir: Option<String>,
}

fn main() {
    let args = Args::parse();
    let file_prefix = args.prefix.unwrap_or(String::from("split_"));
    let output_dir = args.output_dir.unwrap_or(String::from("./"));

    split::split_csv(
        args.input_filepath,
        args.line_count,
        &file_prefix,
        &output_dir,
    )
    .expect("Split csv failed: ");
}
