use std::fmt::{Debug, Display, Formatter};
use crate::interpreter::models::CodeLine;
use crate::interpreter::lexer::assignables::{DigitToken, NameToken};
use crate::interpreter::lexer::methods::MethodCallToken;
use crate::interpreter::utils::logging::TreeViewElement;

#[derive(Clone, PartialEq)]
pub enum AssignableToken {
    Name { value: NameToken },
    Digit { value: DigitToken },
    MethodCall { value: MethodCallToken },
}

impl Display for AssignableToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignableToken::Name { value } => write!(f, "{}", value),
            AssignableToken::Digit { value } => write!(f, "{}", value),
            AssignableToken::MethodCall { value } => write!(f, "{}", value),
        }
    }
}

impl Debug for AssignableToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignableToken::Name { value } => write!(f, "{}", value),
            AssignableToken::Digit { value } => write!(f, "{}", value),
            AssignableToken::MethodCall { value } => write!(f, "{}", value)
        }
    }
}

impl TreeViewElement for AssignableToken {
    fn to_tree_view(&self) -> Vec<String> {
        match self {
            AssignableToken::Name { value } => value.to_tree_view(),
            AssignableToken::Digit { value } => value.to_tree_view(),
            AssignableToken::MethodCall { value } => value.to_tree_view()
        }
    }
}

impl AssignableToken {
    pub fn evaluate(&self) -> u32 {
        match self {
            AssignableToken::Digit { value } => {
                value.evaluate()
            }
            AssignableToken::MethodCall { value } => {
                value.evaluate()
            }
            AssignableToken::Name { value } => {
                value.evaluate()
            }
        }
    }

    pub fn parse(code_line: &CodeLine) -> Option<Self> {
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
        };
    }

    pub fn add_assign(&mut self, rhs: Self) {
        let rhs_value = match rhs {
            AssignableToken::Name { ref value } => {
                value.evaluate()
            }
            AssignableToken::Digit { value } => {
                value.evaluate()
            }
            AssignableToken::MethodCall { value } => {
                value.evaluate()
            }
        };

        *self = AssignableToken::Digit {
            value: DigitToken::new(self.evaluate() + rhs_value)
        }
    }

    pub fn sub_assign(&mut self, rhs: Self) {
        let rhs_value = match rhs {
            AssignableToken::Name { ref value } => {
                value.evaluate()
            }
            AssignableToken::Digit { value } => {
                value.evaluate()
            }
            AssignableToken::MethodCall { value } => {
                value.evaluate()
            }
        };

        *self = AssignableToken::Digit {
            value: DigitToken::new(self.evaluate() - rhs_value)
        }
    }
}