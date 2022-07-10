use crate::interpreter::tokenizer::models::Stackable;

pub trait TreeViewElement {
    fn to_tree_view(&self) -> Vec<String>;
}

impl TreeViewElement for Stackable {
    fn to_tree_view(&self) -> Vec<String> {
        match self {
            Stackable::MethodCallToken { value } => value.to_tree_view(),
            Stackable::VariableToken { value } => value.to_tree_view(),
            Stackable::WhileToken { value } => value.to_tree_view(),
            Stackable::ReturnToken { value } => { value.to_tree_view() }
            Stackable::AdditiveOperatorToken { value } => { value.to_tree_view() }
        }
    }
}