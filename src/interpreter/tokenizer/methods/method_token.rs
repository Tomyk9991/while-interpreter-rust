use std::fmt::{Display, Formatter};
use crate::interpreter::models::{BodyExecutor, CodeLine};
use crate::interpreter::tokenizer::methods::MethodHeaderToken;
use crate::interpreter::tokenizer::models::Token;
use crate::interpreter::tokenizer::scopes::InnerBodyScope;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone)]
pub struct MethodToken {
    pub header_token: MethodHeaderToken,
    pub scope: InnerBodyScope,

    code_lines: Vec<CodeLine>,
    start_index: usize,
    last_visited_line: i32
}

impl MethodToken {
    pub fn execute(&self) -> u32 {
        let body_executor = BodyExecutor {
            scope: self.scope.stack.clone()
        };

        body_executor.execute().unwrap_or(0)
    }
}

impl Display for MethodToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header_token)
    }
}

impl MethodToken {
    pub fn ends_with_return(&self) -> bool {
        // todo
        return self.scope.stack.last().unwrap().is_return_token();
    }

    pub fn new(header_token: MethodHeaderToken, code_lines: Vec<CodeLine>, start_index: usize) -> Self {
        return MethodToken {
            header_token,
            scope: InnerBodyScope::default(),
            code_lines,
            start_index,
            last_visited_line: 0
        };
    }

    pub fn last_line(&self) -> i32 {
        return self.last_visited_line;
    }

    pub fn parse(&mut self, line: &CodeLine) -> Option<Self> {
        self.scope = InnerBodyScope::new(Some(self.header_token.clone()), self.code_lines.clone());

        let mut i = self.start_index;

        while i < self.code_lines.len() {
            let current_line = self.code_lines.get(i).unwrap();
            let token = self.scope.parse(&current_line);

            let header_token = MethodHeaderToken::parse(line);

            if header_token.is_some() {
                pseudo_throw("Can't define a method inside a method".to_string());
                return None;
            }

            i = self.scope.last_visited as usize;
            self.last_visited_line = i as i32;

            if token.is_some() {
                match token.unwrap() {
                    Token::Return { .. } => {
                        break;
                    }
                    _ => { }
                }
            }

            i += 1;
        }

        return Some(self.clone());
    }
}

impl TreeViewElement for MethodToken {
    fn to_tree_view(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("├── Header".to_string());
        lines.push(format!("│  ├── {}", self.header_token.to_tree_view()[0]));
        lines.push("├── Scope:".to_string());

        for line in self.scope.to_tree_view() {
            lines.push(format!("   {}", line));
        }

        return lines;
    }
}