use while_interpreter::interpreter::models::CodeLine;
use while_interpreter::interpreter::lexer::operators::AdditiveOperatorToken;

fn get_tests(operator: &str) -> [(String, bool); 7]{
    let tests =  [
        ("a #= b;", true),
        ("a #= 5", false),
        ("a #= 5;", true),
        ("a #= add(123, 4102);", true),
        ("a #= add(add(21, 31), 4102);", true),
        ("a #= super_long_test();", true),
        ("a #= ;", false),
    ];

    return tests.map(|s| {
        (s.0.replace("#", operator), s.1.clone())
    });
}

#[test]
fn additive_operator() {
    let tests = get_tests("+");

    for test in tests {
        let token = AdditiveOperatorToken::parse(&CodeLine::new_from_line(&test.0));
        assert_eq!(test.1, token.is_some());
    }
}

#[test]
fn subtractive_operator() {
    let tests = get_tests("-");

    for test in tests {
        let token = AdditiveOperatorToken::parse(&CodeLine::new_from_line(&test.0));
        assert_eq!(test.1, token.is_some());
    }
}