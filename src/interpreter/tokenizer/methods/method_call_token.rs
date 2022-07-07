use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::NameToken;
use crate::interpreter::utils::extension_methods::VecNameTokenExtension;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

pub struct MethodCallToken {
    parameters: Vec<NameToken>,
    name: NameToken,
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
        write!(f, "Method Call Token: {}", self.name.value)
    }
}

impl MethodCallToken {
    pub fn evaluate(&self) -> u32 {
        0
    }

    pub fn parse(code_line: &CodeLine) -> Option<MethodCallToken> {
        let split = code_line.line.split(&[' ', ':', '(', ')'][..])
            .filter(|p| !p.is_empty() && !p.trim().is_empty())
            .collect::<Vec<&str>>();

        if !code_line.line.contains("(") || !code_line.line.contains(")") {
            return None;
        }

        if code_line.line.contains("=") {
            return None;
        }

        let name_token = NameToken::parse(split[0]);
        if let None = name_token {
            return None;
        }

        let mut parameters = Vec::new();
        if split.len() > 1 {
            parameters = parse_parameters(split[1].replace(",", ",#").split("#").collect(), code_line);
        }

        return Some(MethodCallToken {
            name: name_token.unwrap(),
            parameters
        });
    }
}


fn parse_parameters(parameters: Vec<&str>, code_line: &CodeLine) -> Vec<NameToken> {
    let mut result = Vec::new();

    let len = parameters.len();
    let mut i = 0;

    for parameter in parameters.borrow() as &[&str] {
        let ending_with_comma = parameter.ends_with(",");

        if i != len - 1 && !ending_with_comma {
            pseudo_throw(format!("Expected a sequence as parameter but got: {}", parameters.join("")));
            return vec![];
        }

        let name = &parameter.replace(",", "");

        let parameter = NameToken::parse(name);
        if let Some(value) = parameter {
            result.push(value);
        } else {
            pseudo_throw(format!("Expected a name for the parameter at line: {}", code_line.line_number));
        }

        i += 1;
    }

    return result;
}