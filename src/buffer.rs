use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::content::Data;
use crate::error::NanoResult;

#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub name: Option<String>,
    pub data: Vec<Data>,
    pub buffer_type: String,
}

impl Buffer {
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
        let content = contents.lines().map(Data::from).collect::<Vec<_>>();
        let file_type = file_name
            .as_ref()
            .extension()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        Ok(Self {
            name: Some(file_name.as_ref().to_string_lossy().to_string()),
            data: content,
            buffer_type: file_type,
        })
    }

    /// Get a row from the file
    /// This will return a row from the file, if it exists.
    /// # Examples
    /// ```rust
    /// use nano::FileDocument;
    /// let file = FileDocument::from_file("Cargo.toml").unwrap();
    /// let row = file.row(0);
    ///```
    pub fn row(&self, index: usize) -> Option<&Data> {
        self.data.get(index)
    }

    /// Get the file type
    pub fn file_type(&self) -> &str {
        &self.buffer_type
    }

    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_document_from_file() {
        let file = Buffer::from_file("Cargo.toml").unwrap();
        assert_eq!(file.name, Some(String::from("Cargo.toml")));
    }

    #[test]
    fn test_file_document_row() {
        let file = Buffer::from_file("Cargo.toml").unwrap();
        assert_eq!(file.row(0), Some(&(Data::from("[package]"))));
    }

    #[test]
    fn test_file_document_file_type() {
        let file = Buffer::from_file("Cargo.toml").unwrap();
        assert_eq!(file.file_type(), "toml");
    }
}
