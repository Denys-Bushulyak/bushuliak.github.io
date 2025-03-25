mod entities;
mod functions;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use entities::{html_file::HtmlFile, validated_args_dto::ValidatedArgsDto};
use functions::read_directory;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args: ValidatedArgsDto = Args::parse().try_into()?;

    let tree: Vec<HtmlFile> = read_directory(&args.input_directory);

    dbg!(tree);
    Ok(())
}
