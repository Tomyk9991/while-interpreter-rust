use while_interpreter::interpreter::models::CodeLine;
use while_interpreter::interpreter::lexer::assignables::NameToken;
use while_interpreter::interpreter::lexer::models::Stackable;
use while_interpreter::interpreter::lexer::operators::AdditiveOperatorToken;
use while_interpreter::interpreter::lexer::scopes::InnerBodyScope;
use while_interpreter::interpreter::lexer::Lexer;
use while_interpreter::interpreter::lexer::while_tokens::{WhileHeaderToken, WhileToken};
use while_interpreter::interpreter::utils::logging::Logger::NoLogger;
use while_interpreter::interpreter::utils::logging::TreeViewElement;

struct CodeLineArrayNestedWhileLoopStackablePair {
    code_lines: Vec<CodeLine>,
    while_count: i32,
    stackable_func_calc: fn() -> Vec<Stackable>
}

struct CodeLineArrayNestedWhileLoopPair {
    code_lines: Vec<CodeLine>,
    while_count: i32,
}

#[test]
fn simple_while() {
    let tests = [
        CodeLineArrayNestedWhileLoopStackablePair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("    a += 1;", 2),
                CodeLine::new("#", 3)
            ],
            while_count: 1,
            stackable_func_calc: || {
                let while_header_token = WhileHeaderToken {
                    against_zero_variable: Some(NameToken::new("a"))
                };
                let mut while_token = WhileToken::new(while_header_token, vec![]);
                while_token.scope = Some(InnerBodyScope::new(None, vec![]));
                while_token.scope.as_mut().unwrap().stack.push(
                    Stackable::AdditiveOperatorToken {
                        value: AdditiveOperatorToken::parse(&CodeLine::new_from_line("a += 1;")).unwrap()
                    });

                return vec![Stackable::WhileToken { value: while_token.clone() }];
            }
        },
        CodeLineArrayNestedWhileLoopStackablePair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("    a += 1;", 2),
                CodeLine::new("    #", 3)
            ],
            while_count: 1,
            stackable_func_calc: || {
                let while_header_token = WhileHeaderToken {
                    against_zero_variable: Some(NameToken::new("a"))
                };
                let mut while_token = WhileToken::new(while_header_token, vec![]);
                while_token.scope = Some(InnerBodyScope::new(None, vec![]));
                while_token.scope.as_mut().unwrap().stack.push(
                    Stackable::AdditiveOperatorToken {
                        value: AdditiveOperatorToken::parse(&CodeLine::new_from_line("a += 1;")).unwrap()
                    });

                return vec![Stackable::WhileToken { value: while_token.clone() }];
            }
        }
    ];

    for test in tests {
        let tokenizer = Lexer::new(NoLogger);
        let scope = tokenizer.tokenize(test.code_lines);

        if test.while_count > 0 {
            assert_eq!(scope.stack.first().unwrap().to_tree_view()[0], (test.stackable_func_calc)().first().unwrap().to_tree_view()[0]);
        } else {
            assert!(scope.stack.is_empty());
        }
    }
}

#[test]
fn extended_while() {
    let tests = [
        CodeLineArrayNestedWhileLoopPair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("    a += 1;", 2),
            ],
            while_count: 0,
        },
        CodeLineArrayNestedWhileLoopPair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("    a += 1;", 2),
                CodeLine::new("    b += 1;", 3),
                CodeLine::new("    c += 1;", 4),
                CodeLine::new("    super_long_method_call();", 5),
                CodeLine::new("#", 5),
            ],
            while_count: 1,
        },
        CodeLineArrayNestedWhileLoopPair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("#", 2),
            ],
            while_count: 1,
        },
        CodeLineArrayNestedWhileLoopPair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("  #", 2),
            ],
            while_count: 1,
        },
    ];

    for test in tests {
        let tokenizer = Lexer::new(NoLogger);
        let scope = tokenizer.tokenize(test.code_lines);

        assert_eq!(scope.stack.len(), test.while_count as usize);
    }
}

#[test]
fn nested_while_loop() {
    let tests = [
        CodeLineArrayNestedWhileLoopPair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("    a += 1;", 2),
            ],
            while_count: 0,
        },
        CodeLineArrayNestedWhileLoopPair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("    while b != 0:", 2),
                CodeLine::new("        while c != 0:", 3),
                CodeLine::new("            while d != 0:", 4),
                CodeLine::new("    #", 5),
                CodeLine::new("#", 6),
            ],
            while_count: 0,
        },
        CodeLineArrayNestedWhileLoopPair {
            code_lines: vec![
                CodeLine::new("while a != 0:", 1),
                CodeLine::new("    while a != 0:", 2),
                CodeLine::new("        while c != 0:", 3),
                CodeLine::new("            while d != 0:", 4),
                CodeLine::new("            #", 5),
                CodeLine::new("        #", 6),
                CodeLine::new("    #", 7),
                CodeLine::new("#", 8),
            ],
            while_count: 4,
        },
    ];

    for test in tests {
        let tokenizer = Lexer::new(NoLogger);
        let scope = tokenizer.tokenize(test.code_lines);

        let mut nesting_count = 0;

        let while_token  = if test.while_count > 0 { scope.stack.first() } else { None };
        if let Some(while_token) = while_token {
            let mut while_token: Option<&WhileToken> = match while_token {
                Stackable::WhileToken { value } => Some(value),
                _ => None
            };

            while while_token.is_some() {
                nesting_count += 1;

                while_token = {
                    let mut found = None;
                    for stackable in &while_token.unwrap().scope.as_ref().unwrap().stack {
                        match stackable {
                            Stackable::WhileToken { value } =>  {
                                found = Some(value);
                                break;
                            },
                            _ => { }
                        }
                    }

                    found
                };
            }

        }


        assert_eq!(test.while_count as usize, nesting_count);
    }
}