use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::models::{Stackable, Token};
use crate::interpreter::utils::logging::TreeViewElement;
use crate::Logger;

pub struct TopLevelScope {
    logger: Logger,
    pub stack: Vec<Stackable>
}

impl TopLevelScope {
    pub fn new(logger: Logger) -> TopLevelScope {
        TopLevelScope {
            logger,
            stack: Vec::new()
        }
    }

    pub fn parse(&self, code_line: &CodeLine) -> Option<Token> {
        return Token::parse(code_line);
    }

    pub fn print(&self) {
        let lines = self.to_tree_view();
        for line in lines {
            self.logger.log(&line);
        }
    }
}

impl TreeViewElement for TopLevelScope {
    fn to_tree_view(&self) -> Vec<String> {
        let mut lines = vec![
            "Program:".to_string(),
            "├── Methods:".to_string(),
        ];

        // todo methods

        lines.push("├── Scope:".to_string());
        for stackable in &self.stack {
            let stackable_lines = stackable.to_tree_view();
            let stackable_lines_count = stackable_lines.len();
            let mut counter = 0;

            for stackable_line in stackable_lines {
                if stackable_lines_count == 1 {
                    lines.push(format!("│  ├── {}", stackable_line));
                } else {
                    if counter == 0 {
                        lines.push(format!("│  {}", stackable_line));
                    } else {
                        lines.push(format!("│  │{}", stackable_line));
                    }
                }

                counter += 1;
            }
        }
        return lines;
    }
}