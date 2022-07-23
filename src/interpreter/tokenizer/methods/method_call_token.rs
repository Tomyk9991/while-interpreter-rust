use std::fmt::{Display, Formatter};
use crate::interpreter::executor_states::RunTime;
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::{DigitToken, NameToken};
use crate::interpreter::tokenizer::models::AssignableToken;
use crate::interpreter::tokenizer::variables::VariableToken;
use crate::interpreter::utils::extension_methods::VecNameTokenExtension;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(PartialEq, Debug)]
pub struct MethodCallToken {
    pub parameters: Vec<AssignableToken>,
    pub name: NameToken,
}

impl Clone for MethodCallToken {
    fn clone(&self) -> Self {
        MethodCallToken {
            parameters: self.parameters.clone(),
            name: self.name.clone(),
        }
    }
}

impl TreeViewElement for MethodCallToken {
    fn to_tree_view(&self) -> Vec<String> {
        vec![format!("Method call: {}, parameters: {}", self.name.value, &self.parameters.to_inline_string())]
    }
}

impl Display for MethodCallToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Method Call Token: {}, Parameters: {}", self.name.value, &self.parameters.to_inline_string())
    }
}

impl MethodCallToken {
    pub fn evaluate(&self) -> u32 {
        // build parameter variables
        // in a method call, the indent level is 1. so we need to add 1 to the indent level.
        let method_token = RunTime::get_method_token(&self.name.value);


        return if let Some(method_token) = method_token {
            let mut parameters: Vec<VariableToken> = Vec::new();

            for (i, assignable) in self.parameters.iter().enumerate() {
                parameters.push(VariableToken::new(
                    method_token.header_token.parameters[i].clone(),
                    AssignableToken::Digit {
                        value: DigitToken::new(assignable.evaluate()),
                    },
                ))
            }

            RunTime::push_parameter_variables(parameters);
            let value = RunTime::get_value_from_method_name(&self.name.value);
            RunTime::pop_variables();

            value
        } else {
            pseudo_throw(format!("Method not found"));
            0
        }
    }

    pub fn parse(code_line: &CodeLine) -> Option<MethodCallToken> {
        if !code_line.line.contains("(") || !code_line.line.contains(")") {
            return None;
        }

        if code_line.line.contains("=") {
            return None;
        }

        let first_bracket = code_line.line.find('(');
        let last_bracket = code_line.line.rfind(')');

        if first_bracket.is_none() || last_bracket.is_none() {
            return None;
        }

        let first_bracket = first_bracket.unwrap();
        let last_bracket = last_bracket.unwrap();

        let parameter_str: &str = &code_line.line[first_bracket + 1..last_bracket];
        let name_str = &code_line.line[0..first_bracket];

        let name_token = NameToken::parse(name_str);
        if let None = name_token {
            return None;
        }

        let mut parameters = Vec::new();
        if parameter_str.trim().len() > 0 {
            parameters = parse_p(parameter_str);

            if parameters.len() == 0 {
                return None;
            }
        }

        return Some(MethodCallToken {
            name: name_token.unwrap(),
            parameters
        });
    }
}

fn parse_p(parameter_string: &str) -> Vec<AssignableToken> {
    let mut parameters: Vec<AssignableToken> = Vec::new();
    let mut individual_parameters: Vec<&str> = Vec::new();
    let mut counter = 0;
    let mut current_start_index = 0;

    for (index, c) in parameter_string.chars().enumerate() {
        match c {
            '(' => counter += 1,
            ')' => counter -= 1,
            ',' => {
                if counter == 0 {
                    let value = &parameter_string[current_start_index..index].trim();

                    if value.is_empty() {
                        pseudo_throw("Parameter can't be empty".to_string());
                        return vec![];
                    }

                    individual_parameters.push(value);
                    current_start_index = index + 1;
                }
            }
            _ => { }
        }
    }

    if counter != 0 {
        pseudo_throw("Expected ')' at method call".to_string());
        return vec![];
    }

    individual_parameters.push(&parameter_string[current_start_index..parameter_string.len()].trim());

    for para in individual_parameters {
        let assignable = AssignableToken::parse(&CodeLine::new_from_line(para));

        if assignable.is_some() {
            parameters.push(assignable.unwrap());
        }
    }


    return parameters;
}