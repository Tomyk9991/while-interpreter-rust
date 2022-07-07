use std::borrow::Borrow;
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::scopes::TopLevelScope;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;
use crate::Logger;

pub struct Tokenizer {
    logger: Logger
}

impl Tokenizer {
    pub fn new(logger: Logger) -> Tokenizer {
        Tokenizer {
            logger
        }
    }

    pub fn tokenize(&self, code_lines: Vec<CodeLine>) -> TopLevelScope {
        let mut scope = TopLevelScope::new(self.logger.clone());

        for code_line in code_lines.borrow() as &Vec<CodeLine> {
            self.logger.log(&format!("{}", code_line));
        }

        for current_line in code_lines.borrow() as &Vec<CodeLine> {
            let token = scope.parse(&current_line);
            if token.is_none() {
                pseudo_throw(format!("Unexpected tokens at line \n\"{}\"", current_line));
                return scope;
            }
            let token = token.unwrap();

            if let Some(stackable) = token.borrow().to_stackable() {
                scope.stack.push(stackable);
                continue;
            }

            if let Some(method_header) = token.borrow().to_method_header() {

            }
        }

        scope.print();

        return scope;
    }
}