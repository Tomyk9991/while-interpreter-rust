use std::fmt::{Display, Formatter};
use crate::interpreter::lexer::operators::{AdditiveOperatorToken, Operator};
use crate::interpreter::lexer::variables::VariableToken;
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
        // It's possible you can manipulate variable of lower and the same indent level, but only
        // if there is only one variables named like the token.name.value, otherwise its shadowed and the highest indent_level is used.
        // get all variables with the same name as the token.name.value
        let mut potential_variables: Vec<usize> = Vec::new();

        for (index, tuple) in self.tokens.iter().enumerate() {
            if tuple.0.name.value == token.name.value {
                potential_variables.push(index);
            }
        }

        // sort the potential variables by their indent level
        potential_variables.sort_by(|a, b| self.tokens[*a].1.cmp(&self.tokens[*b].1));

        if potential_variables.len() == 0 {
            self.tokens.push((token, self.current_indent_level));
            return true;
        }

        let found_value: Option<&mut (VariableToken, u32)> = self.tokens.get_mut(potential_variables[0]);


        if let Some((ref mut var, ref mut indent)) = found_value {
            if self.current_indent_level > *indent {
                self.tokens.push((token, self.current_indent_level));
                return true;
            }

            var.set_assignable_token(token.assignment);
            return false;
        } else {
            self.tokens.push((token, self.current_indent_level));
            return true;
        }
    }

    pub fn update(&mut self, operator_token: AdditiveOperatorToken) {
        let mut potential_variables: Vec<usize> = Vec::new();

        for (index, tuple) in self.tokens.iter().enumerate() {
            if tuple.0.name.value == operator_token.name.value {
                potential_variables.push(index);
            }
        }

        // sort the potential variables by their indent level
        potential_variables.sort_by(|a, b| self.tokens[*a].1.cmp(&self.tokens[*b].1));
        let tuple = self.tokens.get_mut(potential_variables[potential_variables.len() - 1]);

        if tuple.is_none() {
            pseudo_throw(format!("You can't operate on a non existent variable: {}", operator_token.name.value));
            return;
        }

        let (variable_token, _) = tuple.unwrap();

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
        for (variable_token, _) in self.tokens.iter_mut() {
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