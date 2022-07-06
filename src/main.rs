use interpreter::read;
use crate::interpreter::normalize;
use crate::interpreter::tokenizer::scopes::TopLevelScope;
use crate::interpreter::tokenizer::Tokenizer;
use crate::interpreter::utils::env_args_parser;
use crate::interpreter::utils::logging::Logger;

mod interpreter;

fn main() {
    let path = env_args_parser::get_suffix_from_prefix(&["-i", "i"][..]);
    let logger_statement = env_args_parser::get_suffix_from_prefix(&["-log", "log"][..]).unwrap_or("np".to_string());


    if path.is_none() {
        println!("No source file provided. Consider using --i example.while");
        return;
    }


    let logger: Logger = match logger_statement.to_lowercase().as_ref() {
        "nolog" => Logger::NoLogger,
        "log" => Logger::StdLogger,
        "np" => {
            println!("Logging argument not provided. Using default: nolog");
            Logger::NoLogger
        },
        _ => { Logger::NoLogger }
    };


    let mut source_code = read(&(path.unwrap())).unwrap();
    source_code = normalize(&source_code);


    let tokenizer = Tokenizer::new(logger);
    let _scope: TopLevelScope = tokenizer.tokenize(source_code);
    println!("finished");
}
