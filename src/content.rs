pub use unicode_segmentation::UnicodeSegmentation;

/// Content
/// This struct is used to store the content of a row.
/// It is a wrapper around a string, and it also stores the length of the
/// string.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Content {
    pub text: String,
    len: usize,
}

impl From<&str> for Content {
    fn from(text: &str) -> Self {
        Self {
            text: text.graphemes(true).collect(),
            len: text.len(),
        }
    }
}

impl Content {
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
        let content = Content::from("Hello, world!");
        assert_eq!(content.text, "Hello, world!");
        assert_eq!(content.len, 13);
    }

    #[test]
    fn test_content_display() {
        let content = Content::from("Hello, world!");
        assert_eq!(content.display(), "Hello, world!");
    }

    #[test]
    fn test_content_display_range() {
        let content = Content::from("Hello, world!");
        assert_eq!(content.display_range(0, 5), "Hello");
    }

    #[test]
    fn test_content_len() {
        let content = Content::from("Hello, world!");
        assert_eq!(content.len(), 13);
    }

    #[test]
    fn test_content_is_empty() {
        let content = Content::from("Hello, world!");
        assert!(!content.is_empty());
    }

    #[test]
    fn test_content_new() {
        let content = Content::new(String::from("Hello, world!"));
        assert_eq!(content.text, "Hello, world!");
        assert_eq!(content.len, 13);
    }

    #[test]
    fn graphemes_test() {
        let content = Content::from("Hello, world!");
        assert_eq!(content.text, "Hello, world!");
        assert_eq!(content.len, 13);
    }

    #[test]
    fn test_display_range() {
        let content = Content::from("日本語");
        assert_eq!(content.display_range(1, 3), "本語");
    }

    #[test]
    fn test_len_with_emoji() {
        let content = Content::from("😀😃😄😁");
        assert_eq!(content.len(), 16);
    }

    #[test]
    fn test_is_empty() {
        let content = Content::new("".into());
        assert!(content.is_empty());
    }

    #[test]
    fn test_display_with_emoji() {
        let content = Content::from("😀");
        assert_eq!(content.display(), "😀");
        assert_eq!(content.len(), 4);
    }

    #[test]
    fn test_display_range_with_emoji() {
        let content = Content::from("😀😃😄😁");
        assert_eq!(content.display_range(1, 3), "😃😄");
    }
}
