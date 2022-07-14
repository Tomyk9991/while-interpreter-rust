use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::NameToken;
use crate::interpreter::tokenizer::methods::type_token::TypeToken;
use crate::interpreter::utils::extension_methods::VecNameTokenExtension;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, Debug, PartialEq)]
pub struct MethodHeaderToken {
    pub name: NameToken,
    pub parameters: Vec<NameToken>,
    pub return_type: TypeToken
}

impl Display for MethodHeaderToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parameters = if self.parameters.is_empty() {
            "".to_string()
        } else {
            self.parameters.iter().map(|p| format!("{}", p)).collect::<Vec<String>>().join(", ")
        };

        return write!(f, "{} {}({})", self.return_type, self.name.value, parameters);
    }
}

impl MethodHeaderToken {
    pub fn parse(code_line: &CodeLine) -> Option<MethodHeaderToken> {
        let split = code_line.line.split(&[' ', ':', '(', ')'][..])
            .filter(|p| !p.is_empty() && !p.trim().is_empty())
            .collect::<Vec<&str>>();

        if !code_line.line.contains("(") || !code_line.line.contains(")") {
            return None;
        }

        if split.len() < 2 {
            return None;
        }

        let type_token = TypeToken::analyse(split[0]);

        if type_token == TypeToken::Invalid {
            return None;
        }

        if !code_line.line.ends_with(":") {
            pseudo_throw(format!("Expected ':' after method definition, but found: {}", code_line.line.chars().last().unwrap()));
            return None;
        }

        let name_token = NameToken::parse(split[1]);

        if let None = name_token {
            pseudo_throw(format!("Expected method name, but found: {}", code_line.line_number));
            return None;
        }

        let mut parameters = Vec::new();
        if split.len() > 2 {
            let f: Vec<&str> = split[2..][..].to_vec();
            let op_parameters = parse_parameters(f, code_line);

            if op_parameters.is_none() {
                return None;
            }

            parameters = op_parameters.unwrap();
        }

        return Some(MethodHeaderToken {
            return_type: type_token,
            name: name_token.unwrap(),
            parameters
        })
    }
}

fn parse_parameters(parameters: Vec<&str>, code_line: &CodeLine) -> Option<Vec<NameToken>> {
    let mut p = Vec::new();

    let len = parameters.len();
    let mut i = 0;

    for parameter in parameters {
        let ending_with_comma = parameter.ends_with(",");

        if i != len - 1 && !ending_with_comma {
            pseudo_throw(format!("Expected a sequence as parameter at line: {}", code_line.line_number));
            return None;
        }

        let name = &parameter.replace(",", "");

        let parameter = NameToken::parse(name);

        if let Some(value) = parameter {
            p.push(value);
        } else {
            pseudo_throw(format!("Expected a name for the parameter at line: {}", code_line.line_number));
            return None;
        }

        i += 1;
    }

    return Some(p);
}

impl TreeViewElement for MethodHeaderToken {
    fn to_tree_view(&self) -> Vec<String> {
        return vec![format!("name: {}, return: {}, parameters: {}", self.name.value, self.return_type, self.parameters.to_inline_string())];
    }
}

