use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::NameToken;
use crate::interpreter::tokenizer::models::AssignableToken;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Noop
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+="),
            Operator::Sub => write!(f, "-="),
            Operator::Noop => write!(f, "NOOP")
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdditiveOperatorToken {
    pub name: NameToken,
    pub operator: Operator,
    pub rhs_operand: AssignableToken
}

impl Display for AdditiveOperatorToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name, self.operator, self.rhs_operand)
    }
}

impl AdditiveOperatorToken {
    pub fn parse(line: &CodeLine) -> Option<AdditiveOperatorToken> {
        let split = line.line.split(&[' ', ';'][..])
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let name_token = NameToken::parse(split[0]);
        if name_token.is_none() {
            return None;
        }

        let operator = match split[1] {
            "+=" => Operator::Add,
            "-=" => Operator::Sub,
            _ => Operator::Noop
        };

        if operator == Operator::Noop {
            return None;
        }

        let rhs_operand = AssignableToken::parse(&CodeLine::new_from_line(&split[2..].join("")));
        if rhs_operand.is_none() {
            return None;
        }

        if !line.line.ends_with(";") {
            pseudo_throw(format!("Expected \";\" at line {}", line.line_number));
            return None;
        }

        Some(AdditiveOperatorToken {
            name: name_token.unwrap(),
            operator,
            rhs_operand: rhs_operand.unwrap()
        })
    }
}

impl TreeViewElement for AdditiveOperatorToken {
    fn to_tree_view(&self) -> Vec<String> {
        return vec![format!("Operator token: {{name: {}, operator: {}, RHS: {}}}", self.name.value, self.operator, self.rhs_operand.to_tree_view()[0])];
    }
}