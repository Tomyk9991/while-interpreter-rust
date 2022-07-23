mod assignable_token;
mod token;

pub use assignable_token::AssignableToken;
pub use token::Token;

use crate::interpreter::lexer::methods::{MethodCallToken, ReturnToken};
use crate::interpreter::lexer::operators::AdditiveOperatorToken;
use crate::interpreter::lexer::variables::VariableToken;
use crate::interpreter::lexer::while_tokens::WhileToken;

#[derive(Clone, PartialEq, Debug)]
pub enum Stackable {
    MethodCallToken { value: MethodCallToken },
    VariableToken { value: VariableToken },
    WhileToken { value: WhileToken },
    ReturnToken { value: ReturnToken },
    AdditiveOperatorToken { value: AdditiveOperatorToken },
}

impl Stackable {
    pub fn is_return_token(&self) -> bool {
        match self {
            Stackable::ReturnToken { .. } => true,
            _ => false,
        }
    }
}