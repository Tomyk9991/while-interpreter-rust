use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;

#[derive(Clone)]
pub struct WhileEscapeToken;

impl Display for WhileEscapeToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WhileEscapeToken")
    }
}

impl WhileEscapeToken {
    pub fn parse(line: &CodeLine) -> Option<WhileEscapeToken> {
        let split = line.line.split(&[' ', ';'][..])
            .map(|s| s.trim())
            .filter(|&s| !s.is_empty() && !s.trim().is_empty())
            .collect::<Vec<&str>>();

        if split[0].starts_with("#") {
            if split[0].len() == 1 && split.len() == 1 {
                return Some(WhileEscapeToken);
            }

            pseudo_throw(format!("Unexpected tokens after \"#\" at line: {}", line.line_number));
            return None;
        }

        return None;
    }
}