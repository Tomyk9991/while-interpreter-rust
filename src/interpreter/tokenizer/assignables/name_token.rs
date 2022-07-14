use std::fmt::{Display, Formatter};
use regex::Regex;
use crate::interpreter::constants::KEYWORDS;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, Debug, PartialEq)]
pub struct NameToken {
    pub value: String
}

impl TreeViewElement for NameToken {
    fn to_tree_view(&self) -> Vec<String> {
        return vec![format!("{{Value: {}}}", self.value)];
    }
}

impl Display for NameToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name Token: {}", self.value)
    }
}

impl NameToken {
    pub fn new(value: &str) -> Self {
        NameToken {
            value: value.to_string()
        }
    }

    pub fn evaluate(&self) -> u32 {
        return 0;
    }

    pub fn parse(line: &str) -> Option<Self> {
        if KEYWORDS.iter().any(|keyword| line.contains(keyword)) {
            return None;
        }

        let regex = Regex::new("^[a-zA-Z_$][a-zA-Z_$0-9$]*$").unwrap();
        if !regex.is_match(line) {
            return None;
        }

        return Some(NameToken::new(line));
    }
}