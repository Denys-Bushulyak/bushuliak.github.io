use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct HtmlFile {
    pub path: PathBuf,
    pub content: String,
}
