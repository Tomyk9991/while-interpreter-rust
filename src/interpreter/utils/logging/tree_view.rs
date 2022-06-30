
pub trait TreeViewElement {
    fn to_tree_view(&self) -> Vec<String>;
}