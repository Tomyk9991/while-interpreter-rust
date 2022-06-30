use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::models::CodeLine;

pub fn normalize(source_code: &Vec<CodeLine>) -> Vec<CodeLine> {
    let mut source = Vec::new();
    let mut clc: u32 = 1;

    for line in source_code {
        if line.line.trim().starts_with("//") {
            continue;
        }

        if !line.line.is_empty() && !line.line.trim().is_empty() {
            source.push(CodeLine::new(&line.line, clc));
            clc += 1;
        }
    }

    return source;
}

pub fn read(input: &str) -> Result<Vec<CodeLine>, String> {
    match read_lines(input) {
        Ok(lines) => {
            let mut source_code: Vec<CodeLine> = Vec::new();
            let mut i: u32 = 1;

            for l in lines {
                if let Ok(line) = l {
                    source_code.push(CodeLine::new(&line, i));
                }
                i += 1;
            }

            Ok(source_code)
        },
        Err(err) => panic!("Error reading input source code. {}", err)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}