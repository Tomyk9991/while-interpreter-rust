use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::interpreter::tokenizer::assignables::DigitToken;
use crate::interpreter::tokenizer::models::AssignableToken;
use crate::interpreter::tokenizer::models::AssignableToken::Digit;
use crate::interpreter::tokenizer::operators::{AdditiveOperatorToken, Operator};
use crate::interpreter::tokenizer::variables::VariableToken;
use crate::interpreter::utils::extension_methods::VecNameTokenExtension;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;

pub struct VariablesList {
    tokens: Vec<VariableToken>
}

impl VariablesList {
    pub fn new() -> Self {
        VariablesList {
            tokens: Vec::new()
        }
    }

    pub fn add_or_update(&mut self, token: VariableToken) -> bool {
        let found_value = self.tokens.iter_mut().find(|x| x.name.value == token.name.value);
        if found_value.is_none() {
            self.tokens.push(token);
            return true;
        }

        found_value.unwrap().set_assignable_token(token.assignment);

        return false;
    }

    pub fn update(&mut self, operator_token: AdditiveOperatorToken) {
        let variable_token = self.tokens.iter_mut().find(|x| x.name.value == operator_token.name.value);
        if variable_token.is_none() {
            pseudo_throw(format!("You can't operate on a non existent variable: {}", operator_token.name.value));
        }
        
        let mut variable_token = variable_token.unwrap();

        match operator_token.operator {
            Operator::Add => variable_token.get_assignable_mut().add_assign(operator_token.rhs_operand),
            Operator::Sub => variable_token.get_assignable_mut().sub_assign(operator_token.rhs_operand),
            Operator::Noop => pseudo_throw(format!("Noop operator is not supported for variables"))
        }
    }

    pub fn find<P>(&self, predicate: P) -> Option<&VariableToken> where P : Fn(&VariableToken) -> bool {
        for token in &self.tokens {
            let result = predicate(token);
            if result {
                return Some(token)
            }
        }

        None
    }

    pub fn find_mut<P>(&mut self, mut predicate: P) -> Option<&mut VariableToken> where P : FnMut(&mut VariableToken) -> bool {
        for token in self.tokens.iter_mut() {
            let result = predicate(token);
            if result {
                return Some(token)
            }
        }

        None
    }
}

impl Display for VariablesList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tokens.to_multi_line_string())
    }
}