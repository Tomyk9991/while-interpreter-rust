static mut PSEUDO_THROW_MESSAGE: String = String::new();
static mut STATUS: bool = false;

pub fn pseudo_throw(message: String) {
    println!("{}", message);

    unsafe {
        PSEUDO_THROW_MESSAGE = message;
        STATUS = true;
    }
}

pub mod pseudo_status {
    use crate::interpreter::utils::interpreter_watcher::{PSEUDO_THROW_MESSAGE, STATUS};

    pub fn get_status() -> bool {
        unsafe {
            return STATUS;
        }
    }

    pub fn get_message() -> &'static str {
        unsafe {
            return &PSEUDO_THROW_MESSAGE;
        }
    }

    pub fn reset_status() {
        unsafe {
            STATUS = false;
            PSEUDO_THROW_MESSAGE = String::from("");
        }
    }
}