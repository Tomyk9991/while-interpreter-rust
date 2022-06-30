use std::fmt::{Display, Formatter};
use regex::Regex;
use crate::interpreter::constants::KEYWORDS;
use crate::interpreter::tokenizer::models::AssignableToken;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(PartialEq)]
pub struct NameToken<'a> {
    pub value: &'a str
}

impl<'a> TreeViewElement for NameToken<'a> {
    fn to_tree_view(&self) -> Vec<String> {
        return vec![format!("{{Value: {}}}", self.value)];
    }
}

impl<'a> Display for NameToken<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name Token: {}", self.value)
    }
}

impl<'a> NameToken<'a> {
    pub fn new(value: &'a str) -> Self {
        NameToken {
            value
        }
    }

    pub fn evaluate(&self) -> u32 {
        return 0;
    }

    pub fn parse(assignment: &'a str) -> Option<Self> {
        if KEYWORDS.iter().any(|keyword| assignment.contains(keyword)) {
            return None;
        }

        let regex = Regex::new("^[a-zA-Z_$][a-zA-Z_$0-9$]*$").unwrap();
        if !regex.is_match(assignment) {
            return None;
        }

        return Some(NameToken::new(assignment));
    }
}