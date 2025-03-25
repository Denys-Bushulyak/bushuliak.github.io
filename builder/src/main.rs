use std::{error::Error, fmt::Display, path::PathBuf};

use clap::Parser;
use std::fs;

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

#[derive(Debug)]
struct ValidatedArgsDto {
    /// Source folder with markdowns
    r#in: PathBuf,

    /// Destination folder for prepared html
    out: PathBuf,
}

type Directory = PathBuf;

#[derive(Debug)]
enum ArgumentsValidationError {
    InputDirectoryDoesNotExist(Directory),
    OutputDirectoryDoesNotExist(Directory),
    InputShouldBeDirectory(PathBuf),
    OutputShouldBeDirectory(PathBuf),
}

impl Display for ArgumentsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ArgumentsValidationError {}

impl TryFrom<Args> for ValidatedArgsDto {
    type Error = ArgumentsValidationError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let input_directory = if value.r#in.exists() {
            if value.r#in.is_dir() {
                value.r#in
            } else {
                return Err(ArgumentsValidationError::InputShouldBeDirectory(value.r#in));
            }
        } else {
            return Err(ArgumentsValidationError::InputDirectoryDoesNotExist(
                value.r#in,
            ));
        };

        let output_directory = if value.out.exists() {
            if value.out.is_dir() {
                value.out
            } else {
                return Err(ArgumentsValidationError::OutputShouldBeDirectory(value.out));
            }
        } else {
            return Err(ArgumentsValidationError::OutputDirectoryDoesNotExist(
                value.out,
            ));
        };

        Ok(Self {
            r#in: input_directory,
            out: output_directory,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: ValidatedArgsDto = Args::parse().try_into()?;

    let tree: Vec<HtmlFile> = read_directory(args.r#in).collect();

    dbg!(tree);
    Ok(())
}

#[derive(Debug)]
struct HtmlFile {
    path: PathBuf,
    content: String,
}

fn read_directory(dir: PathBuf) -> impl Iterator<Item = HtmlFile> {
    fn visit_dirs(dir: &PathBuf) -> Vec<HtmlFile> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(visit_dirs(&path));
                } else if let Some(ext) = path.extension() {
                    if ext == "md" {
                        if let Ok(content) = fs::read_to_string(&path) {
                            files.push(HtmlFile { path, content });
                        }
                    }
                }
            }
        }
        files
    }

    visit_dirs(&dir).into_iter()
}
