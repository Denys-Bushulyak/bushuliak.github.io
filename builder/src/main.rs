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
    inputDirectory: PathBuf,

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
            inputDirectory: input_directory,
            out: output_directory,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: ValidatedArgsDto = Args::parse().try_into()?;

    let tree: Vec<HtmlFile> = read_directory(&args.inputDirectory)?;

    dbg!(tree);
    Ok(())
}

#[derive(Debug)]
struct HtmlFile {
    path: PathBuf,
    content: String,
}

#[derive(Debug)]
enum ReadDirectoryProblems {
    CanNotReadDirectory(std::io::Error),
    FileNotMarkDown,
}

impl Display for ReadDirectoryProblems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ReadDirectoryProblems {}

fn read_directory(dir_path: &PathBuf) -> Result<Vec<HtmlFile>, ReadDirectoryProblems> {
    fn visit_dirs_fp(dir_path: &PathBuf) -> Result<Vec<HtmlFile>, ReadDirectoryProblems> {
        fs::read_dir(dir_path)
            .and_then(|read_directory| {
                let filter_map = read_directory
                    .flatten()
                    .filter_map(|directory| {
                        let path = directory.path();

                        if path.is_dir() {
                            visit_dirs_fp(&path).ok()
                        } else {
                            let extension = path.extension().unwrap();
                            if extension == "md" {
                                fs::read_to_string(&path)
                                    .map(|content| HtmlFile { path, content })
                                    .map(|v| Vec::from_iter([v]))
                                    .ok()
                            } else {
                                None
                            }
                        }
                    })
                    .flat_map(|v| v.into_iter())
                    .collect();

                Ok(filter_map)
            })
            .map_err(|e| ReadDirectoryProblems::CanNotReadDirectory(e))
    }

    visit_dirs_fp(dir_path)
}
