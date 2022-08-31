use crate::interpreter::models::{BodyExecutor, MethodsList, VariablesList};
use crate::interpreter::lexer::methods::MethodToken;
use crate::interpreter::utils::logging::Logger;
use crate::interpreter::lexer::scopes::TopLevelScope;
use crate::interpreter::lexer::variables::VariableToken;
use crate::interpreter::utils::interpreter_watcher::pseudo_throw;

static mut VARIABLE_LIST: VariablesList = VariablesList::new();
static mut METHODS_LIST: Option<MethodsList> = None;
static mut INITIALIZED: bool = false;



pub struct RunTime {
    logger: Logger,
    body_executor: BodyExecutor
}


impl RunTime {
    pub fn reset() {
        unsafe {
            METHODS_LIST = Some(MethodsList::new());
            VARIABLE_LIST =  VariablesList::new();
            INITIALIZED = false;
        }
    }

    pub fn new(scope: TopLevelScope, logger: Logger) -> Self {
        RunTime::reset();

        for method in &scope.methods {
            let method_name: String = method.header_token.name.value.clone();
            unsafe {
                METHODS_LIST.as_mut().unwrap().insert(method_name, method.clone());
            }
        }

        unsafe {
            INITIALIZED = true;
        }

        RunTime {
            logger,
            body_executor: BodyExecutor {
                scope: scope.stack
            }
        }
    }

    pub fn run(&mut self) {
        self.body_executor.execute();


        unsafe {
            self.logger.log(&format!("{}", VARIABLE_LIST));
        }
    }

    pub fn initialized() -> bool {
        unsafe {
            return INITIALIZED;
        }
    }

    /// This function is used to bring parameters into scope
    pub fn push_parameter_variables(variables: Vec<VariableToken>) {
        unsafe {
            VARIABLE_LIST.current_indent_level += 1;
            for variable in variables {
                VARIABLE_LIST.add_or_update(variable);
            }
        }
    }

    /// This function is used to pop the variables that were pushed by the method.
    pub fn pop_variables() {
        unsafe {
            VARIABLE_LIST.pop_variables();
            VARIABLE_LIST.current_indent_level -= 1;
        }
    }

    pub fn get_method_token(method_name: &str) -> Option<&MethodToken> {
        unsafe {
            let methods = METHODS_LIST.as_mut();
            if let Some(methods) = methods {
                let method_token = methods.get(method_name);
                return method_token;
            }
        }

        return None;
    }


    pub fn get_value_from_method_name(method_name: &str) -> u32 {
        let method_token = RunTime::get_method_token(method_name);
        if let Some(method_token) = method_token {
            return method_token.execute();
        }

        return 0;
    }

    pub fn get_value_from_current_name(variable_name: &str) -> u32 {
        unsafe {
            let variable_token = VARIABLE_LIST.find(|(variable, indent)| variable.name.value == variable_name && *indent == VARIABLE_LIST.current_indent_level);

            if let Some((variable_token, _)) = variable_token {
                return variable_token.assignment.evaluate();
            }

            pseudo_throw(format!("Variable {} not found.", variable_name));

            return 0;
        }
    }

    pub fn get_variable_list() -> &'static mut VariablesList {
        unsafe {
            &mut VARIABLE_LIST
        }
    }
}