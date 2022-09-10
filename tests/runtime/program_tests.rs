use while_interpreter::interpreter::executor_states::RunTime;
use while_interpreter::interpreter::lexer::Lexer;
use while_interpreter::interpreter::lexer::scopes::TopLevelScope;
use while_interpreter::interpreter::models::CodeLine;
use while_interpreter::interpreter::normalize;
use while_interpreter::interpreter::utils::interpreter_watcher::{pseudo_status};
use while_interpreter::interpreter::utils::interpreter_watcher::pseudo_status::get_status;
use while_interpreter::interpreter::utils::logging::Logger;
use while_interpreter::interpreter::utils::logging::Logger::{NoLogger};
use crate::code_line_gen::{gen_code_line, gen_code_line_and_reset};


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
                gen_code_line_and_reset("x = 5;"),
                gen_code_line("a = 3;"),
                gen_code_line("num Add(x ,y):"),
                gen_code_line("    z = x;"),
                gen_code_line("    z += y;"),
                gen_code_line("    return z;"),
                gen_code_line("r = Add(x, a);"),
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
                gen_code_line_and_reset("x = 5;"),
                gen_code_line("a = 3;"),
                gen_code_line("num Add(x, y):"),
                gen_code_line("    z = x;"),
                gen_code_line("    z += y;"),
                gen_code_line("    return z;"),
                gen_code_line(""),
                gen_code_line("num Mul(a, b):"),
                gen_code_line("    result = a;"),
                gen_code_line("    counter = b;"),
                gen_code_line("    counter -= 1;"),
                gen_code_line("    while counter != 0:"),
                gen_code_line("        result += a;"),
                gen_code_line("        counter -= 1;"),
                gen_code_line("    #"),
                gen_code_line(""),
                gen_code_line("    return result;"),
                gen_code_line(""),
                gen_code_line("x += 2;"),
                gen_code_line("y = 3;"),
                gen_code_line("quertz = 1;"),
                gen_code_line(""),
                gen_code_line("while a != 0:"),
                gen_code_line("    while a != 0:"),
                gen_code_line("        a -= 1;"),
                gen_code_line("    #"),
                gen_code_line("    quertz = 5;"),
                gen_code_line("#"),
                gen_code_line(""),
                gen_code_line("y += x;"),
                gen_code_line("z = Add(x, y);"),
                gen_code_line("product = Mul(x, y);"),
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
                gen_code_line_and_reset("x = 5;"),
                gen_code_line("a = 3;"),
                gen_code_line("num IsEqual(actual, expected):"),
                gen_code_line("    target = actual;"),
                gen_code_line("    target -= expected;"),
                gen_code_line("    while target != 0:"),
                gen_code_line("        return 0;"),
                gen_code_line("    #"),
                gen_code_line("    return 1;"),
                gen_code_line("y = IsEqual(x, a);"),
                gen_code_line("z = IsEqual(x, x);"),
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
                gen_code_line_and_reset("x = 5;"),
                gen_code_line("a = 3;"),
                gen_code_line("num IsEqual(actual, expected):"),
                gen_code_line("    target = actual;"),
                gen_code_line("    target -= expected;"),
                gen_code_line("    while target != 0:"),
                gen_code_line("        while target != 0:"),
                gen_code_line("            return 0;"),
                gen_code_line("        #"),
                gen_code_line("    #"),
                gen_code_line("    return 1;"),
                gen_code_line("y = IsEqual(x, a);"),
                gen_code_line("z = IsEqual(x, x);"),
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

#[test]
fn factorial_recursive() {
    let tests = [
        FindableCodeLineStackPair {
            code_lines: vec![
                gen_code_line_and_reset("x = 5;"),
                gen_code_line("num Mul(a, b):"),
                gen_code_line("    result = a;"),
                gen_code_line("    counter = b;"),
                gen_code_line("    counter -= 1;"),
                gen_code_line("    while counter != 0:"),
                gen_code_line("        result += a;"),
                gen_code_line("        counter -= 1;"),
                gen_code_line("    #"),
                gen_code_line(""),
                gen_code_line("    return result;"),
                gen_code_line("num IsEqual(actual, expected):"),
                gen_code_line("    target = actual;"),
                gen_code_line("    target -= expected;"),
                gen_code_line("    while target != 0:"),
                gen_code_line("        return 0;"),
                gen_code_line("    #"),
                gen_code_line("    return 1;"),
                gen_code_line("num Factorial(n):"),
                gen_code_line("    isZero = IsEqual(n, 0);"),
                gen_code_line("    isOne  = IsEqual(n, 1);"),
                gen_code_line("    while isZero != 0:"),
                gen_code_line("        return 1;"),
                gen_code_line("    #"),
                gen_code_line("    while isOne != 0:"),
                gen_code_line("        return 1;"),
                gen_code_line("    #"),
                gen_code_line("    temp = n;"),
                gen_code_line("    temp -= 1;"),
                gen_code_line("    rec = Factorial(temp);"),
                gen_code_line("    f = Mul(n, rec);"),
                gen_code_line("    return f;"),
                gen_code_line("z = Factorial(x);"),

            ],
            results: vec![
                FindableStringValuePair::<u32> {
                    variable_name: String::from("x"),
                    value: 5,
                    findable: true
                },
                FindableStringValuePair::<u32> {
                    variable_name: String::from("z"),
                    value: 120, // 5! = 120
                    findable: true
                },
            ]
        },
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