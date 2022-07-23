use std::fmt::{Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::lexer::methods::{MethodCallToken, MethodHeaderToken, ReturnToken};
use crate::interpreter::lexer::models::Stackable;
use crate::interpreter::lexer::operators::AdditiveOperatorToken;
use crate::interpreter::lexer::variables::VariableToken;
use crate::interpreter::lexer::while_tokens::{WhileEscapeToken, WhileHeaderToken, WhileToken};

pub enum Token {
    Variable { value: VariableToken },
    MethodCall { value: MethodCallToken },
    MethodHeader { value: MethodHeaderToken },
    WhileHeader { value: WhileHeaderToken },
    While { value: WhileToken },
    WhileEscape { value: WhileEscapeToken },
    Return { value: ReturnToken },
    AdditiveOperator { value: AdditiveOperatorToken },
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Variable { value } => write!(f, "{}", value),
            Token::MethodCall { value } => write!(f, "{}", value),
            Token::MethodHeader { value } => write!(f, "{}", value),
            Token::WhileHeader { value } => write!(f, "{}", value),
            Token::While { value } => write!(f, "{}", value),
            Token::WhileEscape { value } => write!(f, "{}", value),
            Token::Return { value } => write!(f, "{}", value),
            Token::AdditiveOperator { value } => write!(f, "{}", value),
        }
    }
}

impl Token {
    pub fn to_while_header_token(&self) -> Option<WhileHeaderToken> {
        match self {
            Token::WhileHeader { value } => Some(value.clone()),
            _ => None,
        }
    }

    pub fn to_method_header(&self) -> Option<MethodHeaderToken> {
        match self {
            Token::MethodHeader { value } => Some(value.clone()),
            _ => None
        }
    }

    pub fn to_while_token(&self) -> Option<WhileToken> {
        match self {
            Token::While { value } => Some(value.clone()),
            _ => None
        }
    }

    pub fn to_while_escape_token(&self) -> Option<WhileEscapeToken> {
        match self {
            Token::WhileEscape { value } => Some(value.clone()),
            _ => None
        }
    }
}

impl Token {
    pub fn to_stackable(&self) -> Option<Stackable> {
        match self {
            Token::Variable { value } => Some(Stackable::VariableToken { value: value.clone() }),
            Token::MethodCall { value } => Some(Stackable::MethodCallToken { value: value.clone() }),
            Token::While { value } => Some(Stackable::WhileToken { value: value.clone() }),
            Token::Return { value } => Some(Stackable::ReturnToken {value: value.clone() }),
            Token::AdditiveOperator { value } => Some(Stackable::AdditiveOperatorToken { value: value.clone() }),
            Token::WhileHeader { .. } => None,
            Token::MethodHeader { .. } => None,
            Token::WhileEscape { .. } => None,
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

        let result = match WhileHeaderToken::parse(line) {
            None => { None }
            Some(v) => {
                Some(Token::WhileHeader {
                    value: v
                })
            }
        };

        if result.is_some() {
            return result;
        }


        let result = match AdditiveOperatorToken::parse(line) {
            None => { None }
            Some(v) => {
                Some(Token::AdditiveOperator {
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