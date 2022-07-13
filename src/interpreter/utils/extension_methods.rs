use crate::interpreter::utils::logging::TreeViewElement;


pub trait VecNameTokenExtension {
    fn to_inline_string(&self) -> String;
}

impl<T> VecNameTokenExtension for Vec<T> where T : TreeViewElement + PartialEq {
    fn to_inline_string(&self) -> String {
        if self.is_empty() {
            return String::from("[]");
        }

        let mut string = String::from("[");
        let last = self.last().unwrap();

        for parameter in self {
            string.push_str(&format!("{}", parameter.to_tree_view()[0]));

            if last != parameter {
                string.push_str(", ");
            }
        }

        string.push_str("]");
        return string;
    }
}