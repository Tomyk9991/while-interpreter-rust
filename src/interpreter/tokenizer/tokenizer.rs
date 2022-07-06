use std::borrow::Borrow;
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::scopes::TopLevelScope;
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

        for code_line in code_lines.borrow() as &Vec<CodeLine> {
            let token = scope.parse(&code_line);

            if let Some(token) = token {
                if let Some(stackable) = token.borrow().to_stackable() {
                    scope.stack.push(stackable);
                }
            }
        }

        scope.print();

        return scope;
    }
}