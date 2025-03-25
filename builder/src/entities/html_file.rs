use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct HtmlFile {
    pub path_to_save: PathBuf,
    pub content: String,
}

impl HtmlFile {
    pub fn new(value: impl Into<(PathBuf, String)>) -> Self {
        let (path_to_save, content) = value.into();
        Self {
            path_to_save,
            content,
        }
    }
}
