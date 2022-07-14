use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::methods::MethodHeaderToken;
use crate::interpreter::tokenizer::methods::type_token::TypeToken;
use crate::interpreter::tokenizer::models::AssignableToken;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnToken {
    pub return_value: Option<AssignableToken>,
    pub header: Option<MethodHeaderToken>
}

impl Display for ReturnToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "return:")
    }
}

impl ReturnToken {
    pub fn new(header: Option<MethodHeaderToken>) -> Self {
        ReturnToken {
            return_value: None,
            header
        }
    }

    pub fn parse(&mut self, line: &CodeLine) -> Option<ReturnToken> {
        let split = line.line.split(&[' ', ';'][..])
            .filter(|&s| !s.is_empty() && !s.trim().is_empty())
            .collect::<Vec<&str>>();

        if split[0] != "return" || self.header.is_none() {
            return None;
        }

        if split.len() == 1 && self.header.as_ref().unwrap().return_type != TypeToken::Void {
            pseudo_throw(format!("The method is not returning the expected value at line: {}", line.line_number));
            return None;
        } else if split.len() > 2 {
            pseudo_throw(format!("Too many returning variables at line: {}", line.line_number));
            return None;
        }

        if split.len() == 2 {
            self.return_value = AssignableToken::parse(&CodeLine::new_from_line(split[1]));
        }

        return Some(self.clone());
    }
}

impl TreeViewElement for ReturnToken {
    fn to_tree_view(&self) -> Vec<String> {
        if self.return_value.is_none() {
            vec![format!("Return token")]
        } else {
            vec![format!("Return: {}", self.return_value.as_ref().unwrap().to_tree_view()[0])]
        }
    }
}