use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::lexer::assignables::NameToken;
use crate::interpreter::lexer::methods::type_token::TypeToken;
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
            let op_parameters = parse_p(&String::from(f.join("")));

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

fn parse_p(parameter_string: &str) -> Option<Vec<NameToken>> {
    let mut parameters: Vec<NameToken> = Vec::new();
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
                        return None;
                    }

                    individual_parameters.push(value);
                    current_start_index = index + 1;
                }
            }
            _ => { }
        }
    }

    if counter != 0 {
        pseudo_throw("Expected ')' at method header".to_string());
        return None;
    }

    individual_parameters.push(&parameter_string[current_start_index..parameter_string.len()].trim());

    for para in individual_parameters {
        let assignable = NameToken::parse(para);

        if assignable.is_some() {
            parameters.push(assignable.unwrap());
        } else {
            return None;
        }
    }


    return Some(parameters);
}

impl TreeViewElement for MethodHeaderToken {
    fn to_tree_view(&self) -> Vec<String> {
        return vec![format!("name: {}, return: {}, parameters: {}", self.name.value, self.return_type, self.parameters.to_inline_string())];
    }
}

