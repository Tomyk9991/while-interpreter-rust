use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::NameToken;
use crate::interpreter::tokenizer::models::{AssignableToken, Stackable};
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;
use crate::StringExtension;

pub struct VariableToken<'a> {
    pub name: NameToken<'a>,
    pub assignment: AssignableToken<'a>
}

impl<'a> Stackable for VariableToken<'a> {}

impl Display for VariableToken<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable: {} = {}", self.name.value, self.assignment.evaluate())
    }
}

impl<'a> TreeViewElement for VariableToken<'a> {
    fn to_tree_view(&self) -> Vec<String> {
        vec![format!("Variable token: {{name: {}, Assignment: {}}}", self.name.value, self.assignment.to_tree_view()[0])]
    }
}

impl<'a> VariableToken<'a> {
    pub fn parse(code_line: &CodeLine) -> Option<VariableToken> {
        let segments = code_line.line.split(&[' ', ';'][..])
            .filter(|p| !p.is_empty())
            .collect::<Vec<&str>>();

        let name_token = NameToken::parse(segments[0]);
        if let None = name_token {
            return None;
        }

        if  segments[1] != "=" {
            return None;
        }

        let assignment_token = AssignableToken::parse(&CodeLine::new_from_line(code_line.line.find_str(&segments[2..].join(""))));
        if !code_line.line.ends_with(";") {
            pseudo_throw(format!("Expected ';' at end of line: {}", code_line.line));
            return None;
        }

        return Some(VariableToken {
            name: name_token.unwrap(),
            assignment: assignment_token.unwrap()
        });
    }
}