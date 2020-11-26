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
    Option(Box<DataType>),
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

    pub fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean => true,
            _ => false,
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

    pub fn is_number(&self) -> bool {
        match self {
            Self::Number => true,
            _ => false,
        }
    }

    pub fn get_option(&self) -> Option<Box<DataType>> {
        match self {
            Self::Option(data_type) => Some(data_type.clone()),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::String => true,
            _ => false,
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            Self::Unknown => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Self::Void => true,
            _ => false,
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
            Self::Option(data_type) => format!("Option<{}>", data_type),
            Self::String => String::from("string"),
            Self::Unknown => String::from("unknown"),
            Self::Void => String::from("void"),
        }
    }
}

impl PartialEq for DataTypes {
    fn eq(&self, other: &Self) -> bool {
        if let Some(self_type) = self.get_array() {
            if let Some(other_type) = other.get_array() {
                return self_type.node == other_type.node;
            }
        } else if let Some((self_arguments, self_return_type)) =
            self.get_function()
        {
            if let Some((other_arguments, other_return_type)) =
                other.get_function()
            {
                if self_arguments.len() == other_arguments.len() {
                    let mut index: usize = 0;

                    for arg in self_arguments.iter() {
                        let other_arg: DataType =
                            other_arguments[index].clone();

                        if arg.node != other_arg.node {
                            return false;
                        }

                        index += 1;
                    }

                    return self_return_type.node == other_return_type.node;
                }
            }
        } else if let Some(self_data) = self.get_hashmap() {
            if let Some(other_data) = other.get_hashmap() {
                if self_data.len() == other_data.len() {
                    for (key, value) in other_data.iter() {
                        if other_data.contains_key(key) {
                            let odata: &DataType = other_data.get(key).unwrap();

                            if value.node == odata.node {
                                continue;
                            }
                        }

                        return false;
                    }

                    return true;
                }
            }
        } else if let Some(self_identifier) = self.get_identifier() {
            if let Some(other_identifier) = other.get_identifier() {
                return self_identifier == other_identifier;
            }
        } else if let Some(self_type) = self.get_option() {
            if let Some(other_type) = other.get_option() {
                return self_type.node == other_type.node;
            }
        }

        (self.is_boolean() && other.is_boolean())
            || (self.is_number() && other.is_number())
            || (self.is_string() && other.is_string())
            || (self.is_unknown() && other.is_unknown())
            || (self.is_void() && other.is_void())
    }
}

impl PartialEq<DataType> for DataTypes {
    fn eq(&self, other: &DataType) -> bool {
        self == &other.node
    }
}
