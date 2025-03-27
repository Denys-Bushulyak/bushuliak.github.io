use std::{
    error::Error,
    fmt::Display,
    fs::{self, DirEntry},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{MarkdownFile, entities::html_file::HtmlFile};

pub fn read_directory(dir_path: PathBuf) -> impl Iterator<Item = (PathBuf, String)> {
    fs::read_dir(dir_path)
        .map(|read_directory| read_directory.flatten())
        .into_iter()
        .flatten()
        .map(recur_read_files)
        .flatten()
}

fn recur_read_files(entry: DirEntry) -> Vec<(PathBuf, String)> {
    let path = entry.path();

    if path.is_dir() {
        read_directory(path).collect()
    } else {
        path.clone()
            .extension()
            .and_then(|extension| {
                if extension == "md" {
                    fs::read_to_string(&path)
                        .map(|content| (path, content))
                        .map(|v| vec![v])
                        .ok()
                } else {
                    Some(Vec::new())
                }
            })
            .unwrap_or_default()
    }
}

fn append_html_extension(file_name: &Path) -> Option<std::ffi::OsString> {
    let mut file_name = file_name.file_stem()?.to_os_string();
    file_name.push(".html");

    Some(file_name)
}

pub fn convert_to_html(
    output_directory: &PathBuf,
    layout_html_file: &str,
    placeholder: &str,
) -> impl Fn(MarkdownFile) -> HtmlFile {
    move |markdown_file: MarkdownFile| {
        let file_name = append_html_extension(&markdown_file.path)
            .unwrap_or(markdown_file.path.as_os_str().to_os_string());

        let mut path_to_save =
            output_directory.join(markdown_file.path.components().skip(2).collect::<PathBuf>());
        path_to_save.set_file_name(file_name);

        let parser = pulldown_cmark::Parser::new(&markdown_file.content);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        let html_output = layout_html_file.replace(&placeholder, &html_output);

        HtmlFile::new((path_to_save, html_output))
    }
}

#[derive(Debug)]
#[allow(unused_attributes, unused)]
pub enum SaveToDiskError {
    ErrorOnSavingFile(std::io::Error),
    CanNotTakeParentDirectoryFrom(PathBuf),
}

impl Display for SaveToDiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl From<std::io::Error> for SaveToDiskError {
    fn from(value: std::io::Error) -> Self {
        SaveToDiskError::ErrorOnSavingFile(value)
    }
}

impl Error for SaveToDiskError {}

pub fn save_to_disk(html_file: HtmlFile) -> Result<(), SaveToDiskError> {
    html_file
        .path_to_save
        .clone()
        .parent()
        .ok_or(SaveToDiskError::CanNotTakeParentDirectoryFrom(
            html_file.path_to_save.clone(),
        ))
        .and_then(|dir| fs::create_dir_all(dir).map_err(Into::into))
        .and_then(|_| {
            std::fs::File::create(html_file.path_to_save)
                .and_then(|mut file| file.write_all(html_file.content.as_bytes()))
                .map_err(Into::into)
        })
}
