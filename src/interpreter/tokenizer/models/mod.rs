mod assignable_token;
mod token;

pub use assignable_token::AssignableToken;
pub use token::Token;

use crate::interpreter::tokenizer::methods::{MethodCallToken, ReturnToken};
use crate::interpreter::tokenizer::operators::AdditiveOperatorToken;
use crate::interpreter::tokenizer::variables::VariableToken;
use crate::interpreter::tokenizer::while_tokens::WhileToken;

#[derive(Clone)]
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