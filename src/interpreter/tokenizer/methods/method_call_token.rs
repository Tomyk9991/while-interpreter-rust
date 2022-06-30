use std::fmt::{Display, Formatter};
use std::slice::Split;
use regex::internal::Input;
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::NameToken;
use crate::interpreter::tokenizer::models::{Stackable, Token};
use crate::interpreter::utils::extension_methods::StringExtension;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

pub struct MethodCallToken<'a> {
    parameters: Vec<NameToken<'a>>,
    name: NameToken<'a>,
}

impl<'a> Stackable for MethodCallToken<'a> {

}

impl TreeViewElement for MethodCallToken<'_> {
    fn to_tree_view(&self) -> Vec<String> {
        vec![format!("{{Method call: {} parameters: {}}}", self.name, to_inline_string(&self.parameters))]
    }
}

impl Display for MethodCallToken<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Method Call Token: {}", self.name.value)
    }
}

impl<'a> MethodCallToken<'a> {
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

fn to_inline_string(parameters: &Vec<NameToken>) -> String {
    if parameters.is_empty() {
        return String::from("[]");
    }

    let mut string = String::from("[");
    let last = parameters.last().unwrap();

    for parameter in parameters {
        string.push_str(&format!("{},", parameter.to_tree_view()[0]));

        if last != parameter {
            string.push_str(", ");
        }
    }

    string.push_str("]");
    return string;
}


fn parse_parameters<'a>(parameters: Vec<&str>, code_line: &'a CodeLine) -> Vec<NameToken<'a>> {
    let mut result = Vec::new();

    let len = parameters.len();
    let mut i = 0;

    for parameter in parameters {
        let ending_with_comma = parameter.ends_with(",");

        if i != len - 1 && !ending_with_comma {
            pseudo_throw(format!("Expected a sequence as parameter at line: {}", code_line.line_number));
        }


        let name = code_line.line.find_str(&parameter.replace(",", "")).unwrap();

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