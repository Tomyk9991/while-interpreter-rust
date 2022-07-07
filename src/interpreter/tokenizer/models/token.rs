use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::methods::{MethodCallToken, MethodHeaderToken};
use crate::interpreter::tokenizer::models::Stackable;
use crate::interpreter::tokenizer::variables::VariableToken;

pub enum Token {
    Variable { value: VariableToken },
    MethodCall { value: MethodCallToken },
    MethodHeader { value: MethodHeaderToken }
}

impl Token {
    pub fn to_method_header(&self) -> Option<MethodHeaderToken> {
        match self {
            Token::MethodHeader { value } => Some(value.clone()),
            _ => None
        }
    }
}

impl Token {
    pub fn to_stackable(&self) -> Option<Stackable> {
        match self {
            Token::Variable { value } => Some(Stackable::VariableToken { value: value.clone() }),
            Token::MethodCall { value } => Some(Stackable::MethodCallToken { value: value.clone() }),
            Token::MethodHeader { .. } => None
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

        if result.is_some() {
            return result;
        }

        let result = match MethodHeaderToken::parse(line) {
            None => { None }
            Some(v) => {
                Some(Token::MethodHeader {
                    value: v
                })
            }
        };

        if result.is_some() {
            return result;
        }

        return None;
    }
}