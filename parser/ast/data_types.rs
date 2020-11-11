use crate::ast::Node;
use std::{collections::HashMap, fmt};

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
    Array(Box<DataType>),
    Boolean,
    Function(Vec<DataType>, Box<DataType>),
    HashMap(HashMap<String, Box<DataType>>),
    Identifier(String),
    Number,
    String,
    Unknown,
    Void,
}

impl DataTypes {
    pub fn get_array(&self) -> Option<Box<DataType>> {
        match self {
            Self::Array(data_type) => Some(data_type.clone()),
            _ => None,
        }
    }

    pub fn get_function(&self) -> Option<(Vec<DataType>, Box<DataType>)> {
        match self {
            Self::Function(arguments, return_type) => {
                Some((arguments.clone(), return_type.clone()))
            }
            _ => None,
        }
    }

    pub fn get_hashmap(&self) -> Option<HashMap<String, Box<DataType>>> {
        match self {
            Self::HashMap(data) => Some(data.clone()),
            _ => None,
        }
    }

    pub fn get_identifier(&self) -> Option<String> {
        match self {
            Self::Identifier(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Array(data_type) => format!("{}[]", data_type),
            Self::Boolean => String::from("boolean"),
            Self::Function(arguments, return_type) => format!(
                "({}) => {}",
                arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                return_type
            ),
            Self::HashMap(data) => format!(
                "{{\n{}\n}}",
                data.iter()
                    .map(|(key, value)| format!(
                        "{}: {}",
                        key,
                        value.to_string()
                    ))
                    .collect::<Vec<String>>()
                    .join(",\n")
            ),
            Self::Identifier(value) => value.clone(),
            Self::Number => String::from("number"),
            Self::String => String::from("string"),
            Self::Unknown => String::from("unknown"),
            Self::Void => String::from("void"),
        }
    }
}
