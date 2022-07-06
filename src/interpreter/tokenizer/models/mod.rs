mod assignable_token;
mod token;

pub use assignable_token::AssignableToken;
pub use token::Token;

use crate::interpreter::tokenizer::methods::MethodCallToken;
use crate::interpreter::tokenizer::variables::VariableToken;

pub enum Stackable {
    MethodCallToken { value: MethodCallToken },
    VariableToken { value: VariableToken },
}