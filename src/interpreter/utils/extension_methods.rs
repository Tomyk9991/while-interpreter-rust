pub trait StringExtension {
    fn find_str(&self, pat: &str) -> Option<&str>;
}

impl StringExtension for String {
    /// Returns an Option to a string-slice reference, if the pattern was found in the source String
    /// # Arguments
    /// * `pat` - A string slice that holds the searching term
    /// # Returns
    /// Returns a String-Slice-Reference to the source
    /// # Examples
    /// ```
    /// let source: String = "Hello world".to_string();
    /// let slice: &str = source.find_str("world").unwrap();
    /// assert!("world", slice);
    /// ```
    fn find_str(&self, pat: &str) -> Option<&str> {
        let i = self.find(pat);

        if let Some(index) = i {
            return Some(&self[index..index + pat.len()]);
        }

        return None;
    }
}