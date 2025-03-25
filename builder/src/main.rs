mod entities;
mod functions;

use std::{error::Error, fs::File, io::Read, path::PathBuf};

use clap::Parser;
use entities::{markdown_file::MarkdownFile, validated_args_dto::ValidatedArgsDto};
use functions::{convert_to_html, read_directory, save_to_disk};

/// Program to parse markdown to html
#[derive(Parser, Debug)]
#[command(about, long_about)]
struct Args {
    /// Source folder with markdowns
    #[arg(short, long)]
    r#in: PathBuf,

    /// Destination folder for prepared html
    #[arg(short, long)]
    out: PathBuf,
}

const LAYOUT_FILE_PATH: &str = "index.html";

fn main() -> Result<(), Box<dyn Error>> {
    let args: ValidatedArgsDto = Args::parse().try_into()?;

    let mut layout_file = String::new();
    File::open(LAYOUT_FILE_PATH)?.read_to_string(&mut layout_file)?;

    read_directory(args.input_directory)
        .map(MarkdownFile::from)
        .map(convert_to_html(&args.output_directory, &layout_file))
        .try_for_each(save_to_disk)
        .map_err(Into::into)
}
