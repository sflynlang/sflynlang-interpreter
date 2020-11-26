use crate::Environment;
use sflynlang_parser::{
    ast::{DataType, DataTypes, Statement},
    Position,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Object {
    position: Position,
    node: Objects,
}

impl Object {
    pub fn new(position: Position, node: Objects) -> Self {
        Self { position, node }
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn get_node(&self) -> Objects {
        self.node.clone()
    }

    pub fn to_data_type(&self) -> DataType {
        match self.get_node() {
            Objects::Boolean(_) => {
                DataType::new(self.get_position(), DataTypes::Boolean)
            }
            Objects::Function {
                arguments,
                body: _,
                return_obj,
                environment: _,
            } => DataType::new(
                self.get_position(),
                DataTypes::Function(
                    arguments.iter().map(|(_key, value)| value.to_data_type()).collect(),
                    Box::new(return_obj.to_data_type()),
                ),
            ),
            Objects::HashMap(data) => {
                let mut data_data: HashMap<String, Box<DataType>> = HashMap::new();

                for (key, value) in data.iter() {
                    data_data.insert(key.clone(), Box::new(value.to_data_type()));
                }

                DataType::new(
                    self.get_position(),
                    DataTypes::HashMap(data_data)
                )
            },
            Objects::Null(value) => DataType::new(self.get_position(), DataTypes::Option(Box::new(value.to_data_type()))),
            Objects::Number(_) => {
                DataType::new(self.get_position(), DataTypes::Number)
            }
            Objects::Return(value) => value.to_data_type(),
            Objects::String(_) => {
                DataType::new(self.get_position(), DataTypes::String)
            }
            Objects::Unknown => DataType::new(self.get_position(), DataTypes::Unknown),
            Objects::Void => DataType::new(self.get_position(), DataTypes::Void),
        }
    }

    pub fn to_string(&self) -> String {
        self.get_node().to_string()
    }
}

#[derive(Clone, Debug)]
pub enum Objects {
    Boolean(bool),
    Function {
        arguments: HashMap<String, Object>,
        body: Vec<Statement>,
        return_obj: Box<Object>,
        environment: Environment,
    },
    HashMap(HashMap<String, Object>),
    Null(Box<Object>),
    Number(i64),
    Return(Box<Object>),
    String(String),
    Unknown,
    Void,
}

impl Objects {
    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_function(
        &self,
    ) -> Option<(HashMap<String, Object>, Vec<Statement>, Box<Object>, Environment)> {
        match self {
            Self::Function {
                arguments,
                body,
                return_obj,
                environment,
            } => Some((
                arguments.clone(),
                body.clone(),
                return_obj.clone(),
                environment.clone(),
            )),
            _ => None,
        }
    }

    pub fn get_hashmap(&self) -> Option<HashMap<String, Object>> {
        match self {
            Self::HashMap(data) => Some(data.clone()),
            _ => None,
        }
    }

    pub fn get_null(&self) -> Option<Box<Object>> {
        match self {
            Self::Null(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_number(&self) -> Option<i64> {
        match self {
            Self::Number(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_return(&self) -> Option<Box<Object>> {
        match self {
            Self::Return(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(value) => Some(value.clone()),
            _ => None,
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

    pub fn is_trusthy(&self) -> bool {
        match self {
            Self::Boolean(value) => value == &true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Boolean(value) => value.to_string(),
            Self::Function {
                arguments,
                body: _,
                return_obj,
                environment: _,
            } => format!(
                "({}) => {}",
                arguments.iter().map(|(_, value)| value.to_string()).collect::<Vec<String>>().join(", "),
                return_obj.to_string()
            ),
            Self::HashMap(data) => format!(
                "{{\n{}\n}}",
                data.iter().map(|(key, value)| format!("{}: {}", key, value.to_string())).collect::<Vec<String>>().join(", ")
            ),
            Self::Null(_) => String::from("null"),
            Self::Number(value) => value.to_string(),
            Self::Return(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::Unknown => String::from("Unknown"),
            Self::Void => String::from("void"),
        }
    }
}
