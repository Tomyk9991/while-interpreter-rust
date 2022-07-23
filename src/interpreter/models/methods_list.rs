use std::collections::HashMap;
use crate::interpreter::lexer::methods::MethodToken;

pub struct MethodsList {
    pub tokens: HashMap<String, MethodToken>
}

impl MethodsList {
    pub fn new() -> Self {
        MethodsList {
            tokens: HashMap::new()
        }
    }

    pub fn get(&self, index :&str) -> Option<&MethodToken> {
        return self.tokens.get(index);
    }

    pub fn insert(&mut self, index: String, value: MethodToken) {
        self.tokens.insert(index.to_string(), value);
    }
}