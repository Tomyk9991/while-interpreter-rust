use interpreter::read;
use crate::interpreter::executor_states::RunTime;
use crate::interpreter::normalize;
use crate::interpreter::tokenizer::scopes::TopLevelScope;
use crate::interpreter::tokenizer::Tokenizer;
use crate::interpreter::utils::env_args_parser;
use crate::interpreter::utils::interpreter_watcher::{pseudo_status, pseudo_throw};
use crate::interpreter::utils::logging::Logger;

mod interpreter;

fn main() {
    let path = env_args_parser::get_suffix_from_prefix(&["-i", "i"][..]).unwrap_or_else(||{
        pseudo_throw("No source file provided. Consider using --i example.while".to_string());
        return String::from("");
    });

    let logger_statement = env_args_parser::get_suffix_from_prefix(&["-log", "log"][..]).unwrap_or("np".to_string());



    let logger: Logger = match logger_statement.to_lowercase().as_ref() {
        "nolog" => Logger::NoLogger,
        "log" => Logger::StdLogger,
        "np" => {
            println!("Logging argument not provided. Using default: nolog");
            Logger::NoLogger
        },
        _ => { Logger::NoLogger }
    };


    let mut source_code = read(&(path)).unwrap();
    source_code = normalize(&source_code);

    let tokenizer = Tokenizer::new(logger.clone());
    let scope: TopLevelScope = tokenizer.tokenize(source_code);

    if pseudo_status::get_status() {
        println!("{}", pseudo_status::get_message());
        return;
    }

    let run_time = RunTime::new(scope, logger.clone());
    run_time.run();
}
