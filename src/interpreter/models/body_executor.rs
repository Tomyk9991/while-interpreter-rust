use crate::interpreter::executor_states::RunTime;
use crate::interpreter::tokenizer::models::Stackable;
use crate::interpreter::utils::extension_methods::VecNameTokenExtension;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;

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
                    let method = RunTime::get_method_list().get(&value.name.value);
                    if let Some(method) = method {
                        value.evaluate();
                    } else {
                        pseudo_throw(format!("No method \"{}\" found.", value.name.value))
                    }
                }
                Stackable::WhileToken { .. } => {}
                Stackable::ReturnToken { value } => {
                    return Some(value.return_value.as_ref().unwrap().evaluate());
                }
            }
        }

        return None;
    }
}
