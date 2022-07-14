use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub enum TypeToken {
    Void = 0,
    Num = 1,
    Invalid = 2
}

impl Display for TypeToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeToken::Void => write!(f, "void"),
            TypeToken::Num => write!(f, "num"),
            TypeToken::Invalid => write!(f, "invalid")
        }
    }
}

impl TypeToken {
    pub fn analyse(line: &str) -> TypeToken {
        return match line {
            "void" => TypeToken::Void,
            "num" => TypeToken::Num,
            _ => TypeToken::Invalid
        }
    }
}