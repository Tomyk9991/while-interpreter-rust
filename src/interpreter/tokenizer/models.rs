use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
use crate::interpreter::models::CodeLine;
use crate::interpreter::tokenizer::assignables::{DigitToken, NameToken};
use enum_variant_type::EnumVariantType;
use crate::interpreter::tokenizer::methods::MethodCallToken;
use crate::interpreter::tokenizer::variables::VariableToken;
use crate::interpreter::utils::logging::TreeViewElement;

pub trait Stackable {

}

pub enum AssignableToken<'a> {
    Name { value: NameToken<'a> },
    Digit { value: DigitToken },
    MethodCall { value: MethodCallToken<'a> }
}

impl<'a> Add for AssignableToken<'a> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let value = DigitToken::new(self.evaluate() + rhs.evaluate());
        return AssignableToken::Digit { value };
    }
}

impl<'a> Sub for AssignableToken<'a> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = DigitToken::new(self.evaluate() - rhs.evaluate());
        return AssignableToken::Digit { value };
    }
}

impl<'a> Debug for AssignableToken<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignableToken::Name { value } => write!(f, "{}", value),
            AssignableToken::Digit { value} => write!(f, "{}", value),
            AssignableToken::MethodCall { value } => write!(f, "{}", value)
        }
    }
}

impl<'a> TreeViewElement for AssignableToken<'a> {
    fn to_tree_view(&self) -> Vec<String> {
        match self {
            AssignableToken::Name { value } => value.to_tree_view(),
            AssignableToken::Digit { value } => value.to_tree_view(),
            AssignableToken::MethodCall { value } => value.to_tree_view()
        }
    }
}

impl<'a> AssignableToken<'a> {
    pub fn evaluate(&self) -> u32 {
        match self {
            AssignableToken::Name { value } => {
                value.evaluate()
            }
            AssignableToken::Digit { value } => {
                value.evaluate()
            }
            AssignableToken::MethodCall { value } => {
                value.evaluate()
            }
        }
    }

    pub fn parse(code_line: &'a CodeLine) -> Option<Self> {
        let name_assignment_token = NameToken::parse(&code_line.line);
        if let Some(value) = name_assignment_token {
            return Some(AssignableToken::Name { value });
        }

        let method_assignment_token = MethodCallToken::parse(code_line);
        if let Some(value) = method_assignment_token {
            return Some(AssignableToken::MethodCall { value });
        }

        let digit_assignment_token = DigitToken::parse(&code_line.line);

        return match digit_assignment_token {
            Some(value) => {
                Some(AssignableToken::Digit { value })
            }
            None => {
                None
            }
        }
    }
}

pub enum Token<'a> {
    Variable { value : VariableToken<'a> },
    MethodCall { value: MethodCallToken<'a> },
}

impl<'a> Token<'a> {
    pub fn ends_with_semicolon(&self, phrase: &str) -> bool {
        phrase.ends_with(';')
    }

    pub fn parse(&self, line: &'a CodeLine) -> Option<Token> {
        match self {
            Token::Variable { value } => {
                match VariableToken::parse(line) {
                    None => { None }
                    Some(v) => {
                        Some(Token::Variable {
                            value: v
                        })
                    }
                }
            }
            Token::MethodCall { value } => {
                match MethodCallToken::parse(line) {
                    None => { None }
                    Some(v) => {
                        Some(Token::MethodCall {
                            value: v
                        })
                    }
                }
            }
        }
    }
}