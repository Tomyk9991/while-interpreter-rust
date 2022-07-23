use while_interpreter::interpreter::models::CodeLine;
use while_interpreter::interpreter::lexer::methods::{MethodHeaderToken, ReturnToken};
use while_interpreter::interpreter::lexer::models::{AssignableToken, Stackable};
use while_interpreter::interpreter::lexer::operators::AdditiveOperatorToken;
use while_interpreter::interpreter::lexer::Lexer;
use while_interpreter::interpreter::lexer::variables::VariableToken;
use while_interpreter::interpreter::utils::logging::Logger::NoLogger;

#[test]
fn inner_body() {
    let tests = [
        (vec![
            "void main():",
            "    a = 5;",
            "    b = 5;",
            "    return;"
        ], 1, vec![
            Stackable::VariableToken { value: VariableToken::parse(&CodeLine::new_from_line("a = 5;")).unwrap() },
            Stackable::VariableToken { value: VariableToken::parse(&CodeLine::new_from_line("b = 5;")).unwrap() },
            Stackable::ReturnToken { value: ReturnToken::new(MethodHeaderToken::parse(&CodeLine::new_from_line("void main():"))) }
        ]),
        (vec![
            "void main():",
            "    a = 5;",
            "    b = 5;",
        ], 0, vec![
            Stackable::VariableToken { value: VariableToken::parse(&CodeLine::new_from_line("a = 5;")).unwrap() },
            Stackable::VariableToken { value: VariableToken::parse(&CodeLine::new_from_line("b = 5;")).unwrap() },
            Stackable::ReturnToken { value: ReturnToken::new(MethodHeaderToken::parse(&CodeLine::new_from_line("void main():"))) }
        ]),
        (vec![
            "void blubbi(b):",
            "    e += 5;",
            "    return e;"
        ], 1, vec![
            Stackable::AdditiveOperatorToken { value: AdditiveOperatorToken::parse(&CodeLine::new_from_line("e += 5;")).unwrap() },
            Stackable::ReturnToken { value: ReturnToken {
                header: MethodHeaderToken::parse(&CodeLine::new_from_line("void blubbi(b):")),
                return_value: AssignableToken::parse(&CodeLine::new_from_line("e"))
            }}
        ])
    ];

    for pair in tests {
        let tokenizer = Lexer::new(NoLogger);
        let lines: Vec<CodeLine> = pair.0.iter().enumerate().map(|(i, l)| CodeLine::new(l, (i + 1) as u32)).collect();
        let scope = tokenizer.tokenize(lines);


        if scope.methods.len() > 0 {
            for (index, current_stackable) in scope.methods[0].scope.stack.iter().enumerate() {
                assert_eq!(*current_stackable, pair.2[index]);
            }
        }

        assert_eq!(scope.methods.len(), pair.1);
    }
}