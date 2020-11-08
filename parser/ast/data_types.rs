use crate::ast::Node;
use std::fmt;

pub type DataType = Node<DataTypes>;

impl DataType {
    pub fn to_string(&self) -> String {
        self.node.to_string()
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node.to_string())
    }
}

#[derive(Clone, Debug)]
pub enum DataTypes {
    Boolean,
    Identifier(String),
    Number,
    String,
    Unknown,
    Void,
}

impl DataTypes {
    pub fn get_identifier(&self) -> Option<String> {
        match self {
            Self::Identifier(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Boolean => String::from("boolean"),
            Self::Identifier(value) => value.clone(),
            Self::Number => String::from("number"),
            Self::String => String::from("string"),
            Self::Unknown => String::from("unknown"),
            Self::Void => String::from("void"),
        }
    }
}
