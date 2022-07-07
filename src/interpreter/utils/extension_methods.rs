use crate::interpreter::tokenizer::assignables::NameToken;
use crate::interpreter::utils::logging::TreeViewElement;

pub trait StringExtension {
    fn find_str(&self, pat: &str) -> Option<&str>;
}

pub trait VecNameTokenExtension {
    fn to_inline_string(&self) -> String;
}

impl VecNameTokenExtension for Vec<NameToken> {
    fn to_inline_string(&self) -> String {
        if self.is_empty() {
            return String::from("[]");
        }

        let mut string = String::from("[");
        let last = self.last().unwrap();

        for parameter in self {
            string.push_str(&format!("{}", parameter.to_tree_view()[0]));

            if last != parameter {
                string.push_str(", ");
            }
        }

        string.push_str("]");
        return string;
    }
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