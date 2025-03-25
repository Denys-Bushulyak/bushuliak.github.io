use std::{error::Error, fmt::Display, path::PathBuf};

use crate::Args;

#[derive(Debug)]
pub(crate) struct ValidatedArgsDto {
    /// Source folder with markdowns
    pub input_directory: PathBuf,

    /// Destination folder for prepared html
    pub output_directory: PathBuf,
}
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

        let output_directory = value.out;

        Ok(Self {
            input_directory,
            output_directory,
        })
    }
}
#[derive(Debug)]
#[allow(unused_attributes, unused)]
pub enum ArgumentsValidationError {
    InputDirectoryDoesNotExist(PathBuf),
    OutputDirectoryDoesNotExist(PathBuf),
    InputShouldBeDirectory(PathBuf),
    OutputShouldBeDirectory(PathBuf),
}

impl Display for ArgumentsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ArgumentsValidationError {}
