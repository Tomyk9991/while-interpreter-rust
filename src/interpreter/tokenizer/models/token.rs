use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::methods::MethodCallToken;
use crate::interpreter::tokenizer::models::Stackable;
use crate::interpreter::tokenizer::variables::VariableToken;

pub enum Token {
    Variable { value: VariableToken },
    MethodCall { value: MethodCallToken },
}

impl Token {
    pub fn to_stackable(&self) -> Option<Stackable> {
        match self {
            Token::Variable { value } => Some(Stackable::VariableToken { value: value.clone() }),
            Token::MethodCall { value } => Some(Stackable::MethodCallToken { value: value.clone() })
        }
    }
}

impl Token {
    pub fn parse(line: &CodeLine) -> Option<Token> {
        let result = match VariableToken::parse(line) {
            None => { None }
            Some(v) => {
                Some(Token::Variable {
                    value: v
                })
            }
        };

        if result.is_some() {
            return result;
        }

        let result = match MethodCallToken::parse(line) {
            None => { None }
            Some(v) => {
                Some(Token::MethodCall {
                    value: v
                })
            }
        };

        return if result.is_some() {
            result
        } else {
            None
        }
    }
}