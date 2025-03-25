use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::HtmlFile;

pub fn read_directory(dir_path: &PathBuf) -> Vec<HtmlFile> {
    fs::read_dir(dir_path)
        .map(|read_directory| read_directory.flatten())
        .map(Iterator::collect::<Vec<DirEntry>>)
        .iter()
        .flatten()
        .map(read_html_files(&transform_to_html_file))
        .flat_map(|v| v.into_iter())
        .collect::<Vec<HtmlFile>>()
}

fn transform_to_html_file(path: &PathBuf, content: &str) -> HtmlFile {
    HtmlFile {
        path: path.clone(),
        content: content.to_string(),
    }
}

fn read_html_files<F>(transform: &F) -> impl Fn(&DirEntry) -> Vec<HtmlFile>
where
    F: Fn(&PathBuf, &str) -> HtmlFile,
{
    return |directory: &DirEntry| {
        let path = directory.path();

        if path.is_dir() {
            read_directory(&path)
        } else {
            let extension = path.extension().unwrap();
            if extension == "md" {
                fs::read_to_string(&path)
                    .map(|content| transform(&path, &content))
                    .map(|v| vec![v])
                    .unwrap()
            } else {
                vec![]
            }
        }
    };
}
