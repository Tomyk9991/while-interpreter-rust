use std::{env};

use interpreter::read;
use crate::interpreter::normalize;
use crate::interpreter::utils::extension_methods::StringExtension;
use crate::interpreter::utils::logging::Logger;

mod interpreter;

fn main() {
    let path = check_for_input_source_code().unwrap_or_default();
    let mut source_code = read(&path).unwrap();
    source_code = normalize(&source_code);

    let logger: Logger = Logger::NoLogger;
    // let tokenizer = Tokenizer::new(logger);
}

fn check_for_input_source_code() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        let split: Vec<&str> = arg.split(&['-', ' '][..]).filter(|p| !p.is_empty()).collect();

        
        if split.len() == 2 && (split[0] == "i" || split[0] == "-i"){
            return Some(split[1].to_string());
        }
    }

    return None;
}
