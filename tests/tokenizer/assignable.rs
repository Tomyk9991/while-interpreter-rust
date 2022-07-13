use while_interpreter::interpreter::models::CodeLine;
use while_interpreter::interpreter::tokenizer::assignables::NameToken;
use while_interpreter::interpreter::tokenizer::methods::MethodCallToken;

#[test]
fn name_token() {
    let string_result_pair = [
        ("hallo", true),
        ("5", false),
        ("a5232a", true),
        ("5a", false),
        ("^^a", false),
        ("while", false),
    ];

    for pair in string_result_pair {
        let token = NameToken::parse(pair.0);

        if pair.1 {
            assert_eq!(pair.0, token.unwrap().value)
        }
    }
}

#[test]
fn method_call_token() {
    let string_result_pair = [
        ("add(1, 3)", true),
        ("add(a, b)", true),
        ("add()", true),
        ("subtract(5, 5, ,2, 5, 3, 123)", false),
        ("subtract(5, 5, a,2, 5, 3, 123)", true),
        ("subtract(substract(subtract(3, 2), substract(3, 1)), a)", true),
        ("subtract(substract(subtract3, 2), substract(3, 1)), a)", false),
        ("super_long_method()", true),
        ("while)", false),
        ("while(", false),
        ("5.5.5", false),
        ("-1", false),
        ("14", false),
        ("-1231212351", false),
    ];

    for pair in string_result_pair {
        let token = MethodCallToken::parse(&CodeLine::new_from_line(pair.0));
        assert_eq!(pair.1, token.is_some())
    }
}