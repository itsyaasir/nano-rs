pub use unicode_segmentation::UnicodeSegmentation;

/// Content
/// This struct is used to store the content of a row.
/// It is a wrapper around a string, and it also stores the length of the
/// string.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Data {
    pub text: String,
    len: usize,
}

impl From<&str> for Data {
    fn from(text: &str) -> Self {
        Self {
            text: text.graphemes(true).collect(),
            len: text.len(),
        }
    }
}

impl Data {
    pub fn new(text: String) -> Self {
        Self {
            text: text.clone(),
            len: text.len(),
        }
    }
    /// Display the content
    /// This will return the content as a string.
    pub fn display(&self) -> String {
        self.text.clone()
    }

    /// Get a range of the content
    /// This will return a range of the content, if it exists.
    pub fn display_range(&self, start: usize, end: usize) -> String {
        self.text
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect()
    }

    /// Create a new Content
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the content is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_from_str() {
        let content = Data::from("Hello, world!");
        assert_eq!(content.text, "Hello, world!");
        assert_eq!(content.len, 13);
    }

    #[test]
    fn test_content_display() {
        let content = Data::from("Hello, world!");
        assert_eq!(content.display(), "Hello, world!");
    }

    #[test]
    fn test_content_display_range() {
        let content = Data::from("Hello, world!");
        assert_eq!(content.display_range(0, 5), "Hello");
    }

    #[test]
    fn test_content_len() {
        let content = Data::from("Hello, world!");
        assert_eq!(content.len(), 13);
    }

    #[test]
    fn test_content_is_empty() {
        let content = Data::from("Hello, world!");
        assert!(!content.is_empty());
    }

    #[test]
    fn test_content_new() {
        let content = Data::new(String::from("Hello, world!"));
        assert_eq!(content.text, "Hello, world!");
        assert_eq!(content.len, 13);
    }

    #[test]
    fn graphemes_test() {
        let content = Data::from("Hello, world!");
        assert_eq!(content.text, "Hello, world!");
        assert_eq!(content.len, 13);
    }

    #[test]
    fn test_display_range() {
        let content = Data::from("日本語");
        assert_eq!(content.display_range(1, 3), "本語");
    }

    #[test]
    fn test_len_with_emoji() {
        let content = Data::from("😀😃😄😁");
        assert_eq!(content.len(), 16);
    }

    #[test]
    fn test_is_empty() {
        let content = Data::new("".into());
        assert!(content.is_empty());
    }

    #[test]
    fn test_display_with_emoji() {
        let content = Data::from("😀");
        assert_eq!(content.display(), "😀");
        assert_eq!(content.len(), 4);
    }

    #[test]
    fn test_display_range_with_emoji() {
        let content = Data::from("😀😃😄😁");
        assert_eq!(content.display_range(1, 3), "😃😄");
    }
}
