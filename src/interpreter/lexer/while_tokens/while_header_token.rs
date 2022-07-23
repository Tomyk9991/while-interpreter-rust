use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::lexer::assignables::NameToken;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, Debug, PartialEq)]
pub struct WhileHeaderToken {
    // x != 0
    // The while language only can use x != 0, so no need for real boolean algebra
    pub against_zero_variable: Option<NameToken>
}

impl Display for WhileHeaderToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.against_zero_variable {
            Some(ref variable) => write!(f, "while {} != 0", variable),
            None => write!(f, "while")
        }
    }
}

impl WhileHeaderToken {
    pub fn new(against_zero_variable: Option<NameToken>) -> WhileHeaderToken {
        WhileHeaderToken {
            against_zero_variable
        }
    }

    pub fn parse(code_line: &CodeLine) -> Option<Self> {
        let split = code_line.line.split(&[' ', ':'][..])
            .filter(|p| !p.is_empty() && !p.trim().is_empty())
            .collect::<Vec<&str>>();

        if split[0] != "while" {
            return None;
        }

        let name_token = NameToken::parse(split[1]);

        if name_token.is_none() {
            pseudo_throw(format!("Expected a name at line: {}", code_line.line));
            return None;
        }

        if split[2] != "!=" {
            pseudo_throw(format!("Expected a != at line: {}", code_line.line));
            return None;
        }

        if split[3] != "0" {
            pseudo_throw(format!("Expected a \"0\" as comparer at line: {}", code_line.line));
            return None;
        }

        if !code_line.line.ends_with(":") {
            pseudo_throw(format!("Expected a \":\" at line: {}", code_line.line));
            return None;
        }

        Some(WhileHeaderToken::new(name_token))
    }
}

impl TreeViewElement for WhileHeaderToken {
    fn to_tree_view(&self) -> Vec<String> {
        vec![format!("Header: {{while target: {}}}", self.against_zero_variable.as_ref().unwrap().to_tree_view()[0])]
    }
}
