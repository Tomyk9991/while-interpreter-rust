use std::fmt::{Display, Formatter};
use crate::interpreter::executor_states::RunTime;
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::NameToken;
use crate::interpreter::tokenizer::models::AssignableToken;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, PartialEq, Debug)]
pub struct VariableToken {
    pub name: NameToken,
    pub assignment: AssignableToken
}

impl Display for VariableToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable: {} = {}", self.name.value, self.assignment.evaluate())
    }
}

impl TreeViewElement for VariableToken {
    fn to_tree_view(&self) -> Vec<String> {
        if RunTime::initialized() {
            vec![format!("Variable token: {{name: {} = {}\tAssignment: {}}}", self.name.value, self.assignment.evaluate(), self.assignment.to_tree_view()[0])]
        } else {
            vec![format!("Variable token: {{name: {}, Assignment: {}}}", self.name.value, self.assignment.to_tree_view()[0])]
        }
    }
}

impl VariableToken {
    pub fn get_assignable_mut(&mut self) -> &mut AssignableToken {
        &mut self.assignment
    }

    pub fn new(name: NameToken, assignment: AssignableToken) -> Self {
        VariableToken {
            name,
            assignment
        }
    }


    pub fn set_assignable_token(&mut self, assignable_token: AssignableToken) {
        self.assignment = assignable_token;
    }

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

        let sub_string = &segments[2..].join("");

        let assignment_token = AssignableToken::parse(&CodeLine::new_from_line(sub_string));
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