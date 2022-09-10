use crate::interpreter::executor_states::RunTime;
use crate::interpreter::lexer::models::Stackable;

pub struct BodyExecutor {
    pub scope: Vec<Stackable>
}

impl BodyExecutor {
    pub fn execute(&self) -> Option<u32> {
        for stackable in &self.scope {
            match stackable {
                Stackable::VariableToken { value } => {
                    RunTime::get_variable_list().add_or_update(value.clone());
                }
                Stackable::AdditiveOperatorToken { value } => {
                    RunTime::get_variable_list().update(value.clone());
                }
                Stackable::MethodCallToken { ref value } => {
                    value.evaluate();
                }
                Stackable::WhileToken { value } => {
                    let option = value.evaluate();
                    if option.is_some() {
                        return option;
                    }
                }
                Stackable::ReturnToken { value } => {
                    return Some(value.return_value.as_ref().unwrap().evaluate());
                }
            }
        }

        return None;
    }
}
