use std::fmt::format;
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use crate::interpreter::models::VariablesList;
use crate::interpreter::tokenizer::models::{AssignableToken, Stackable};
use crate::interpreter::utils::logging::Logger;
use crate::interpreter::tokenizer::scopes::TopLevelScope;

pub struct RunTime {
    logger: Logger,
    scope: TopLevelScope
}

impl RunTime {
    pub fn new(scope: TopLevelScope, logger: Logger) -> Self {
        RunTime {
            scope,
            logger
        }
    }

    pub fn run(&self) {
        let current_scope = &self.scope.stack;
        let mut variables = RunTime::get_var_list_instance().lock().unwrap();

        for stackable in current_scope {
            match stackable {
                Stackable::VariableToken { value } => {
                    variables.add_or_update(value.clone());
                }
                Stackable::AdditiveOperatorToken { value } => {
                    variables.update(value.clone());
                }
                Stackable::MethodCallToken { .. } => {}
                Stackable::WhileToken { .. } => {}
                Stackable::ReturnToken { .. } => {}
            }
        }

        self.logger.log("Variable states:");
        self.logger.log(&format!("{}", variables));
    }

    fn get_var_list_instance() -> &'static Mutex<VariablesList> {
        static mut VAR_LIST: MaybeUninit<Mutex<VariablesList>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            VAR_LIST.as_mut_ptr().write(Mutex::new(VariablesList::new()));
        });

        unsafe {
            &*VAR_LIST.as_mut_ptr()
        }
    }

    pub fn get_value_from_current_name(variable_name: &str) -> u32 {
        let var_list = RunTime::get_var_list_instance().lock().unwrap();
        let variable_token = var_list.find(|t| t.name.value == variable_name);

        if let Some(variable_token) = variable_token {
            match variable_token.assignment {
                AssignableToken::Digit { ref value } => {
                    return value.evaluate();
                }
                _ => {}
            };

            // recursive lock calls -> single lock? locks himself, because not freeing before
            return variable_token.assignment.evaluate();
        }

        return 0;
    }
}