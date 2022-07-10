use std::borrow::Borrow;
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::methods::MethodToken;
use crate::interpreter::tokenizer::models::Stackable;
use crate::interpreter::tokenizer::scopes::TopLevelScope;
use crate::interpreter::tokenizer::while_tokens::WhileToken;
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

        let mut i = 0;

        while i < code_lines.len() {
            let mut current_line = code_lines.get(i).unwrap();
            let token = scope.parse(&current_line);

            if token.is_none() {
                i += 1;
                continue;
            }

            let token = token.unwrap();

            if let Some(stackable) = token.borrow().to_stackable() {
                scope.stack.push(stackable);
                i += 1;
                continue;
            }

            if let Some(method_header) = token.borrow().to_method_header() {
                let method_header_string = method_header.to_string();
                let mut method = MethodToken::new(method_header, code_lines.clone(), current_line.line_number as usize);

                if (i + 1) >= code_lines.len() {
                    pseudo_throw(format!("Method can't be empty at line: {}", i));
                    return scope;
                }

                current_line = code_lines.get(i + 1).unwrap();
                let method_token = method.parse(&current_line);

                if method_token.is_none() {
                    return scope;
                }

                let last_line = method.last_line() as usize;
                scope.methods.push(method);

                if !method_token.unwrap().ends_with_return() {
                    pseudo_throw(format!("Method {} must end with a return at line: {}", method_header_string ,(i + 1)));
                    scope.methods.remove(scope.methods.len() - 1);
                    return scope;
                }

                i = last_line;
            }

            if let Some(while_header_token) = token.borrow().to_while_header_token() {
                let mut removed_while_from_stack = false;
                let mut while_token = WhileToken::new(while_header_token, code_lines.clone());

                let mut j = i + 1;

                while j < code_lines.len() {
                    let current_line = code_lines.get(j).unwrap();
                    let inner_token = while_token.parse(current_line);

                    if inner_token.is_some() {
                        if let Some(stacked_while_token) = inner_token.as_ref().unwrap().to_while_token() {
                            if !stacked_while_token.escape_token_found {
                                pseudo_throw(format!("Missing escape token at line: {}", while_token.clone().scope.unwrap().last_visited));
                                // todo: vielleicht falsch. Wobei Operationen nach einem pseudo_throw sowieso egal sind
                                scope.stack.pop();
                                removed_while_from_stack = true;
                                break;
                            }
                        }
                    }
                    j = while_token.scope.as_ref().unwrap().last_visited as usize;

                    if inner_token.is_some() {
                        if inner_token.as_ref().unwrap().to_while_escape_token().is_some() {
                            while_token.escape_token_found = true;
                            break;
                        }
                    }

                    j += 1;
                }

                scope.stack.push(Stackable::WhileToken { value : while_token.clone() });
                i = j;

                if !while_token.escape_token_found {
                    if !removed_while_from_stack {
                        pseudo_throw(format!("Missing escape token at line: {}", while_token.scope.unwrap().last_visited));
                        scope.stack.pop();
                    }

                    break;
                }
            }

            i += 1;
        }

        scope.print();

        return scope;
    }
}