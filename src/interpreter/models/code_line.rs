use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq)]
pub struct CodeLine {
    pub line: String,
    pub line_number: u32
}

impl Debug for CodeLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\t \"{}\"", self.line_number, self.line)
    }
}
impl Display for CodeLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\t \"{}\"", self.line_number, self.line)
    }
}

impl CodeLine {
    pub fn new(line: &str, line_number: u32) -> Self {
        CodeLine {
            line: line.to_string(),
            line_number
        }
    }

    pub fn new_from_line(line: &str) -> Self {
        return CodeLine::new(line, 0);
    }
}