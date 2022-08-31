use while_interpreter::interpreter::models::CodeLine;
use while_interpreter::interpreter::lexer::methods::MethodHeaderToken;
use while_interpreter::interpreter::lexer::Lexer;
use while_interpreter::interpreter::utils::logging::Logger::NoLogger;

#[test]
fn method_header() {
    let tests = [
        ("void hallo():", true),
        ("void this_is_a_long_method():", true),
        ("void this_is_a_long_method()", false),
        ("num hallo():", true),
        ("void hallo():", true),
        ("num Add(x ,y):", true),
        ("num againLongMethodName(a, b):", true),
        ("num againLongMethodName(a, 5):", false),
        ("void againLongMethodName(a, b324s):", true),
        ("void againLongMethodName(a, 1231a):", false),
    ];

    for test in tests {
        let token = MethodHeaderToken::parse(&CodeLine::new_from_line(test.0));
        assert_eq!(test.1, token.is_some());
    }
}

#[test]
fn complete_method() {
    let tests = [
        (vec![
            "void hallo1():",
            "    a = 5;",
            "    return;"
         ], 1),
        (vec![
             "void hallo2():",
             "    a = 5;"
         ], 0),
        (vec![
            "void hallo3():",
            "    a = 5;",
            "    return;",
            "void welt():",
            "    b = 6;",
            "    return;"
        ], 2),
        (vec![
            "void hallo4():",
            "void welt():",
            "    b = 6;",
            "    return;"
        ], 0),
        (vec![
            "num welt5():",
            "    b = 6;",
            "    return 5;",
        ], 1),
        (vec![
            "num welt6():",
            "    b = 6;",
            "    return welt();",
        ], 1)
    ];

    for pair in tests {
        let tokenizer = Lexer::new(NoLogger);
        let lines: Vec<CodeLine> = (pair.0 as Vec<&str>).iter().enumerate().map(|(i, l)| CodeLine::new(l, (i + 1) as u32)).collect();
        let scope = tokenizer.tokenize(lines);

        assert_eq!(pair.1, scope.methods.len());
    }
}