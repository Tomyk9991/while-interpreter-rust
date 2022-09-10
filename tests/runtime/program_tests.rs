use while_interpreter::interpreter::executor_states::RunTime;
use while_interpreter::interpreter::lexer::Lexer;
use while_interpreter::interpreter::lexer::scopes::TopLevelScope;
use while_interpreter::interpreter::models::CodeLine;
use while_interpreter::interpreter::normalize;
use while_interpreter::interpreter::utils::interpreter_watcher::{pseudo_status, pseudo_throw};
use while_interpreter::interpreter::utils::interpreter_watcher::pseudo_status::get_status;
use while_interpreter::interpreter::utils::logging::Logger;
use while_interpreter::interpreter::utils::logging::Logger::{NoLogger};


struct StringValuePair<T> {
    variable_name: String,
    value: T
}

struct FindableStringValuePair<T> {
    variable_name: String,
    value: T,
    findable: bool
}

struct CodeLineStackPair {
    code_lines: Vec<CodeLine>,
    results: Vec<StringValuePair<u32>>
}

struct FindableCodeLineStackPair {
    code_lines: Vec<CodeLine>,
    results: Vec<FindableStringValuePair<u32>>
}

#[test]
fn variable_set() {
    let tests = [
        CodeLineStackPair {
            code_lines: vec![
                CodeLine::new("x = 0;", 1),
                CodeLine::new("b = 5125;", 2)
            ],
            results: vec![
                StringValuePair::<u32> {
                    variable_name: String::from("x"),
                    value: 0
                },
                StringValuePair::<u32> {
                    variable_name: String::from("b"),
                    value: 5125
                }
            ]
        },
        CodeLineStackPair {
            code_lines: vec![
                CodeLine::new("askdlkaslk = 223902;", 1),
                CodeLine::new("das_ist_mein_toller_name = 231;", 2)
            ],
            results: vec![
                StringValuePair::<u32> {
                    variable_name: String::from("askdlkaslk"),
                    value: 223902
                },
                StringValuePair::<u32> {
                    variable_name: String::from("das_ist_mein_toller_name"),
                    value: 231
                }
            ]
        }
    ];

    for test in tests {
        let tokenizer = Lexer::new(NoLogger);
        let scope: TopLevelScope = tokenizer.tokenize(test.code_lines.clone());

        if pseudo_status::get_status() {
            println!("{}", pseudo_status::get_message());
            return;
        }

        let mut run_time = RunTime::new(scope, NoLogger);
        run_time.run();

        for result in &test.results {
            let actual_result = RunTime::get_value_from_current_name(&result.variable_name);
            let expected = result.value;

            assert_eq!(actual_result, expected);
        }
    }
}

#[test]
fn method_call() {
    let tests = [
        FindableCodeLineStackPair {
            code_lines: vec![
                CodeLine::new("x = 5;", 1),
                CodeLine::new("a = 3;", 2),
                CodeLine::new("num Add(x ,y):", 3),
                CodeLine::new("    z = x;", 4),
                CodeLine::new("    z += y;", 5),
                CodeLine::new("    return z;", 6),
                CodeLine::new("r = Add(x, a);", 7),
            ],
            results: vec![
                FindableStringValuePair::<u32> {
                    variable_name: String::from("x"),
                    value: 5,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("a"),
                    value: 3,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("z"),
                    value: 5 + 3,
                    findable: false
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("r"),
                    value: 5 + 3,
                    findable: true
                }
            ]
        },
        FindableCodeLineStackPair {
            code_lines: vec![
                CodeLine::new("x = 5;", 1),
                CodeLine::new("a = 3;", 2),
                CodeLine::new("num Add(x, y):", 3),
                CodeLine::new("    z = x;", 4),
                CodeLine::new("    z += y;", 5),
                CodeLine::new("    return z;", 6),
                CodeLine::new("", 7),
                CodeLine::new("num Mul(a, b):", 8),
                CodeLine::new("    result = a;", 9),
                CodeLine::new("    counter = b;", 10),
                CodeLine::new("    counter -= 1;", 11),
                CodeLine::new("    while counter != 0:", 12),
                CodeLine::new("        result += a;", 13),
                CodeLine::new("        counter -= 1;", 14),
                CodeLine::new("    #", 15),
                CodeLine::new("", 16),
                CodeLine::new("    return result;", 17),
                CodeLine::new("", 18),
                CodeLine::new("x += 2;", 19),
                CodeLine::new("y = 3;", 20),
                CodeLine::new("quertz = 1;", 21),
                CodeLine::new("", 22),
                CodeLine::new("while a != 0:", 23),
                CodeLine::new("    while a != 0:", 24),
                CodeLine::new("        a -= 1;", 25),
                CodeLine::new("    #", 26),
                CodeLine::new("    quertz = 5;", 27),
                CodeLine::new("#", 28),
                CodeLine::new("", 29),
                CodeLine::new("y += x;", 30),
                CodeLine::new("z = Add(x, y);", 31),
                CodeLine::new("product = Mul(x, y);", 32),
            ],
            results: vec![
                FindableStringValuePair::<u32> {
                    variable_name: String::from("x"),
                    value: 7,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("a"),
                    value: 0,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("y"),
                    value: 3 + 7,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("quertz"),
                    value: 5,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("z"),
                    value: 17,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("product"),
                    value: 70,
                    findable: true
                }
            ]
        }
    ];

    for test in tests {
        pseudo_status::reset_status();
        RunTime::reset();

        let source_code = normalize(&test.code_lines);
        let lexer = Lexer::new(NoLogger);
        let scope: TopLevelScope = lexer.tokenize(source_code);

        if pseudo_status::get_status() {
            println!("Status failed: not continuing");
            println!("{}", pseudo_status::get_message());
            return;
        }

        let mut run_time = RunTime::new(scope, NoLogger);
        run_time.run();


        for result in &test.results {
            let actual_result = RunTime::get_value_from_current_name(&result.variable_name);
            let expected = result.value;

            if result.findable == true {
                assert_eq!(actual_result, expected);
            } else {
                assert!(get_status());
            }
        }
    }
}

#[test]
fn while_return() {
    let tests = [
        FindableCodeLineStackPair {
            code_lines: vec![
                CodeLine::new("x = 5;", 1),
                CodeLine::new("a = 3;", 2),
                CodeLine::new("num IsEqual(actual, expected):", 3),
                CodeLine::new("    target = actual;", 4),
                CodeLine::new("    target -= expected;", 5),
                CodeLine::new("    while target != 0:", 6),
                CodeLine::new("        return 0;", 7),
                CodeLine::new("    #", 8),
                CodeLine::new("    return 1;", 9),
                CodeLine::new("y = IsEqual(x, a);", 10),
                CodeLine::new("z = IsEqual(x, x);", 11),
            ],
            results: vec![
                FindableStringValuePair::<u32> {
                    variable_name: String::from("x"),
                    value: 5,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("a"),
                    value: 3,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("y"),
                    value: 0,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("z"),
                    value: 1,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("target"),
                    value: 0,
                    findable: false
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("expected"),
                    value: 0,
                    findable: false
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("actual"),
                    value: 0,
                    findable: false
                },
            ]
        },
        FindableCodeLineStackPair {
            code_lines: vec![
                CodeLine::new("x = 5;", 1),
                CodeLine::new("a = 3;", 2),
                CodeLine::new("num IsEqual(actual, expected):", 3),
                CodeLine::new("    target = actual;", 4),
                CodeLine::new("    target -= expected;", 5),
                CodeLine::new("    while target != 0:", 6),
                CodeLine::new("        while target != 0:", 7),
                CodeLine::new("            return 0;", 8),
                CodeLine::new("        #", 9),
                CodeLine::new("    #", 10),
                CodeLine::new("    return 1;", 11),
                CodeLine::new("y = IsEqual(x, a);", 12),
                CodeLine::new("z = IsEqual(x, x);", 13),
            ],
            results: vec![
                FindableStringValuePair::<u32> {
                    variable_name: String::from("x"),
                    value: 5,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("a"),
                    value: 3,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("y"),
                    value: 0,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("z"),
                    value: 1,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("target"),
                    value: 0,
                    findable: false
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("expected"),
                    value: 0,
                    findable: false
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("actual"),
                    value: 0,
                    findable: false
                },
            ]
        }
    ];

    for test in tests {
        pseudo_status::reset_status();
        RunTime::reset();

        let source_code = normalize(&test.code_lines);
        let lexer = Lexer::new(Logger::NoLogger);
        let scope: TopLevelScope = lexer.tokenize(source_code);

        if pseudo_status::get_status() {
            println!("Status failed: not continuing");
            println!("{}", pseudo_status::get_message());
            return;
        }


        let mut run_time = RunTime::new(scope, NoLogger);
        run_time.run();


        for result in &test.results {
            let actual_result = RunTime::get_value_from_current_name(&result.variable_name);
            let expected = result.value;

            if result.findable == true {
                assert_eq!(actual_result, expected);
            } else {
                assert!(get_status());
            }
        }
    }
}