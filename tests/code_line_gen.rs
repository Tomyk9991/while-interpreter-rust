use while_interpreter::interpreter::models::CodeLine;

static mut COUNTER: u32 = 1;

pub fn reset_counter() {
    unsafe { COUNTER = 1; }
}

pub fn gen_code_line(line: &str) -> CodeLine {
    return CodeLine::new(line, unsafe { COUNTER });
}

pub fn gen_code_line_and_reset(line: &str) -> CodeLine {
    reset_counter();
    return CodeLine::new(line, unsafe { COUNTER });
}