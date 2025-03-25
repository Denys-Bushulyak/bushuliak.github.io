use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct MarkdownFile {
    pub path: PathBuf,
    pub content: String,
}
impl MarkdownFile {
    pub fn create(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }
}

impl<T> From<T> for MarkdownFile
where
    T: Into<(PathBuf, String)>,
{
    fn from(value: T) -> Self {
        let (path, content) = value.into();
        MarkdownFile::create(path, content)
    }
}
