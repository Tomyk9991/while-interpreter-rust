use std::fmt::{Display, Formatter};
use regex::Regex;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone)]
pub struct DigitToken {
    value: u32
}

impl Display for DigitToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Digit Token: {}", self.value)
    }
}

impl TreeViewElement for DigitToken {
    fn to_tree_view(&self) -> Vec<String> {
        return vec![format!("{{Value: {}}}", self.value)];
    }
}

impl DigitToken {
    pub fn new(value: u32) -> Self {
        DigitToken {
            value
        }
    }

    pub fn evaluate(&self) -> u32 {
        return self.value;
    }

    pub fn parse(assignment: &str) -> Option<DigitToken> {
        let regex = Regex::new("^[0-9]+$").unwrap();
        if !regex.is_match(assignment) {
            return None;
        }

        return Some(
            DigitToken::new(assignment.parse::<u32>().unwrap())
        );
    }
}