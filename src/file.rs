use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::content::Content;
use crate::error::NanoResult;

#[derive(Debug, Clone, Default)]
pub struct FileDocument {
    pub file_name: Option<String>,
    pub content: Vec<Content>,
}

impl FileDocument {
    /// Open a file and create a new FileDocument
    /// This will open a file and create a new FileDocument from it.
    ///
    /// # Errors
    /// This function will return an error if the file cannot be opened or read.
    /// # Examples
    /// ```
    /// use nano::FileDocument;
    /// let file = FileDocument::from_file("Cargo.toml").unwrap();
    ///
    pub fn from_file<P: AsRef<Path>>(file_name: P) -> NanoResult<Self> {
        let mut file = File::open(file_name.as_ref())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let rows = contents.lines().map(Content::from).collect::<Vec<_>>();
        Ok(Self {
            file_name: Some(file_name.as_ref().to_string_lossy().to_string()),
            content: rows,
        })
    }

    /// Get a row from the file
    /// This will return a row from the file, if it exists.
    /// # Examples
    /// ```
    /// use nano::FileDocument;
    /// let file = FileDocument::from_file("Cargo.toml").unwrap();
    /// let row = file.row(0);
    /// ```
    ///
    ///
    pub fn row(&self, index: usize) -> Option<&Content> {
        self.content.get(index)
    }

    /// Check if the file is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_document_from_file() {
        let file = FileDocument::from_file("Cargo.toml").unwrap();
        assert_eq!(file.file_name, Some(String::from("Cargo.toml")));
    }

    #[test]
    fn test_file_document_row() {
        let file = FileDocument::from_file("Cargo.toml").unwrap();
        assert_eq!(file.row(0), Some(&(Content::from("[package]"))));
    }

    #[test]
    fn test_file_document_is_empty() {
        let file = FileDocument::from_file("Cargo.toml").unwrap();
        assert!(file.is_empty());
    }
}
