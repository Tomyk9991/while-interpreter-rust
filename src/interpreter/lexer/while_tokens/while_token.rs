use std::fmt::{Display, Formatter};
use crate::interpreter::lexer::methods::MethodHeaderToken;
use crate::interpreter::models::{BodyExecutor, CodeLine};
use crate::interpreter::lexer::models::Token;
use crate::interpreter::lexer::scopes::InnerBodyScope;
use crate::interpreter::lexer::while_tokens::WhileHeaderToken;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, Debug, PartialEq)]
pub struct WhileToken {
    pub header_token: WhileHeaderToken,
    pub scope: Option<InnerBodyScope>,
    pub escape_token_found: bool,
    pub method_header_token: Option<MethodHeaderToken>,

    code_lines: Vec<CodeLine>
}

impl WhileToken {
    pub fn evaluate(&self) -> Option<u32> {
        if let Some(against_zero_variable) = &self.header_token.against_zero_variable {
            if let Some(scope) = &self.scope {

                let body_executor: BodyExecutor = BodyExecutor {
                    scope: scope.stack.clone()
                };

                while against_zero_variable.evaluate() != 0 {
                    if let Some(value) = body_executor.execute() {
                        return Some(value);
                    }
                }
            }
        }
        
        None
    }
}

impl Display for WhileToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {}", self.header_token)
    }
}

impl WhileToken {
    pub fn new(header_token: WhileHeaderToken, method_header_token: Option<MethodHeaderToken>, code_lines: Vec<CodeLine>) -> Self {
        WhileToken {
            header_token,
            scope: None,
            method_header_token,
            escape_token_found: false,
            code_lines
        }
    }

    pub fn parse(&mut self, code_line: &CodeLine) -> Option<Token> {
        if self.scope.is_none() {
            self.scope = Some(InnerBodyScope::new(self.method_header_token.clone(), self.code_lines.clone()));
        }

        return self.scope.as_mut().unwrap().parse(code_line);
    }
}

impl TreeViewElement for WhileToken {
    fn to_tree_view(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(        "├── While Token:".to_string());
        lines.push(format!("   ├── {}", self.header_token.to_tree_view()[0]));
        lines.push(        "   └── Scope:".to_string());

        if self.scope.is_some() {
            let temp_lines = self.scope.as_ref().unwrap().to_tree_view();

            for temp_line in temp_lines {
                lines.push(format!("      {}", temp_line));
            }
        }

        return lines;
    }
}