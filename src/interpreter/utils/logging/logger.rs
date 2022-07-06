#[derive(Clone)]
pub enum Logger {
    NoLogger,
    StdLogger,
}

impl Logger {
    pub fn log(&self, message: &str) {
        match self {
            Logger::NoLogger => {}
            Logger::StdLogger => println!("{}", message),
        }
    }
}