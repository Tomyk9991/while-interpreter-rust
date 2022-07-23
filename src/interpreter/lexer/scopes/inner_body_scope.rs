use crate::interpreter::models::CodeLine;
use crate::interpreter::lexer::methods::{MethodHeaderToken, ReturnToken};
use crate::interpreter::lexer::models::{Stackable, Token};
use crate::interpreter::lexer::operators::AdditiveOperatorToken;
use crate::interpreter::lexer::variables::VariableToken;
use crate::interpreter::lexer::while_tokens::{WhileEscapeToken, WhileHeaderToken, WhileToken};
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, Debug, PartialEq)]
pub struct InnerBodyScope {
    pub stack: Vec<Stackable>,
    pub last_visited: i32,
    header: Option<MethodHeaderToken>,
    code_lines: Vec<CodeLine>,
}

impl Default for InnerBodyScope {
    fn default() -> Self {
        InnerBodyScope {
            stack: Vec::new(),
            last_visited: 0,
            header: None,
            code_lines: Vec::new(),
        }
    }
}

impl InnerBodyScope {
    pub fn new(header: Option<MethodHeaderToken>, code_lines: Vec<CodeLine>) -> Self {
        InnerBodyScope {
            stack: Vec::new(),
            last_visited: 0,
            header,
            code_lines,
        }
    }

    pub fn parse(&mut self, line: &CodeLine) -> Option<Token> {
        let variable_token = VariableToken::parse(line);

        self.last_visited = (line.line_number - 1) as i32;

        if variable_token.is_some() {
            let variable_token = variable_token.unwrap();

            let var = Stackable::VariableToken {
                value: variable_token.clone()
            };

            self.stack.push(var);

            return Some(Token::Variable {
                value: variable_token.clone()
            });
        }

        let while_header_token = WhileHeaderToken::parse(line);
        if while_header_token.is_some() {
            let mut while_token = WhileToken::new(while_header_token.unwrap(), self.code_lines.clone());


            let mut i = line.line_number as i32;

            while i < self.code_lines.len() as i32 {
                let current_line = &self.code_lines[i as usize];
                let token = while_token.parse(current_line);
                i = while_token.scope.as_ref().unwrap().last_visited;

                self.last_visited = i;

                if token.is_some() {
                    if token.unwrap().to_while_escape_token().is_some() {
                        while_token.escape_token_found = true;
                        break;
                    }
                }

                i += 1;
            }

            self.stack.push(Stackable::WhileToken {
                value: while_token.clone()
            });

            return Some(Token::While {
                value: while_token.clone()
            });
        }

        let return_token = ReturnToken::new(self.header.clone()).parse(line);
        if return_token.is_some() {
            let return_token = return_token.unwrap();
            self.stack.push(Stackable::ReturnToken {
                value: return_token.clone()
            });

            return Some(Token::Return {
                value: return_token.clone()
            });
        }

        let operator_token = AdditiveOperatorToken::parse(line);
        if operator_token.is_some() {
            let operator_token = operator_token.unwrap();
            self.stack.push(Stackable::AdditiveOperatorToken {
                value: operator_token.clone()
            });

            return Some(Token::AdditiveOperator {
                value: operator_token.clone()
            });
        }

        let while_escape_token = WhileEscapeToken::parse(line);

        return if while_escape_token.is_some() {
            Some(Token::WhileEscape {
                value: while_escape_token.unwrap()
            })
        } else {
            None
        };
    }
}


impl TreeViewElement for InnerBodyScope {
    fn to_tree_view(&self) -> Vec<String> {
        let mut lines = Vec::new();

        for stackable in &self.stack {
            let temp_lines = stackable.to_tree_view();

            for (i, temp_line) in temp_lines.iter().enumerate() {
                if temp_lines.len() == 1 {
                    if i == temp_lines.len() - 1 {
                        lines.push(format!("└─ {}", temp_line));
                    } else {
                        lines.push(format!("├── {}", temp_line));
                    }
                } else {
                    lines.push(temp_line.clone());
                }
            }
        }

        return lines;
    }
}