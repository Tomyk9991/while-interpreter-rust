use std::fmt::{Display, Formatter};
use std::slice::Iter;
use crate::interpreter::tokenizer::operators::{AdditiveOperatorToken, Operator};
use crate::interpreter::tokenizer::variables::VariableToken;
use crate::interpreter::utils::extension_methods::VecNameTokenExtension;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;

pub struct VariablesList {
    //Represents a Variable, indent-level pair
    tokens: Vec<(VariableToken, u32)>,
    pub current_indent_level: u32,
}

impl VariablesList {
    pub const fn new() -> Self {
        VariablesList {
            tokens: Vec::new(),
            current_indent_level: 0,
        }
    }

    pub fn add_or_update(&mut self, token: VariableToken) -> bool {
        // there is a problem, when the name of a parameter is the same as a already defined variable.
        // in this case a new variable is created, but the old one is not updated.
        // Design the language, so variables from other scopes are not even visible.
        // You can only manipulate the variables in the current scope, and the variables that were passed a parameter.

        let mut found_value = self.tokens.iter_mut().find(|(variable, indent_level)| {
            variable.name.value == token.name.value && *indent_level == self.current_indent_level
        });

        return if let Some((ref mut var, ref mut indent)) = found_value {
            var.set_assignable_token(token.assignment);
            false
        } else {
            self.tokens.push((token, self.current_indent_level));
            true
        }
    }

    pub fn update(&mut self, operator_token: AdditiveOperatorToken) {
        let tuple = self.tokens.iter_mut().find(|(variable, indent)| {
            variable.name.value == operator_token.name.value && *indent == self.current_indent_level
        });

        if tuple.is_none() {
            pseudo_throw(format!("You can't operate on a non existent variable: {}", operator_token.name.value));
        }

        let (variable_token, indent) = tuple.unwrap();

        match operator_token.operator {
            Operator::Add => variable_token.get_assignable_mut().add_assign(operator_token.rhs_operand),
            Operator::Sub => variable_token.get_assignable_mut().sub_assign(operator_token.rhs_operand),
            Operator::Noop => pseudo_throw(format!("Noop operator is not supported for variables"))
        }
    }

    pub fn pop_variables(&mut self) {
        // variables getting popped with the same indent level as the current indent level.


        for i in (0..self.tokens.len()).rev() {
            if self.tokens[i].1 == self.current_indent_level {
                self.tokens.remove(i);
            }
        }
    }

    pub fn find<P>(&self, predicate: P) -> Option<(&VariableToken, &u32)> where P: Fn((&VariableToken, &u32)) -> bool {
        for (variable_token, indent_level) in &self.tokens {
            let result = predicate((variable_token, indent_level));
            if result {
                return Some((variable_token, indent_level));
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn find_mut<P>(&mut self, mut predicate: P) -> Option<&mut VariableToken> where P: FnMut(&mut VariableToken) -> bool {
        for (variable_token, indent_level) in self.tokens.iter_mut() {
            let result = predicate(variable_token);
            if result {
                return Some(variable_token);
            }
        }

        None
    }
}

impl Display for VariablesList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tokens.iter().map(|tuple| tuple.0.clone()).collect::<Vec<VariableToken>>().to_multi_line_string())
    }
}