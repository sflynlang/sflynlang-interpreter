use sflynlang_parser::{
    ast::{DataType, DataTypes},
    Position,
};

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
            Objects::Number(_) => {
                DataType::new(self.get_position(), DataTypes::Number)
            }
            Objects::Return(value) => {
                if let Some(value) = value {
                    value.to_data_type()
                } else {
                    DataType::new(self.get_position(), DataTypes::Void)
                }
            }
            Objects::String(_) => {
                DataType::new(self.get_position(), DataTypes::String)
            }
            Objects::Void => {
                DataType::new(self.get_position(), DataTypes::Void)
            }
        }
    }

    pub fn to_string(&self) -> String {
        self.get_node().to_string()
    }
}

#[derive(Clone, Debug)]
pub enum Objects {
    Boolean(bool),
    Number(i64),
    Return(Option<Box<Object>>),
    String(String),
    Void,
}

impl Objects {
    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_number(&self) -> Option<i64> {
        match self {
            Self::Number(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_return(&self) -> Option<Option<Box<Object>>> {
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

    pub fn to_string(&self) -> String {
        match self {
            Self::Boolean(value) => value.to_string(),
            Self::Number(value) => value.to_string(),
            Self::Return(value) => {
                if let Some(value) = value {
                    value.to_string()
                } else {
                    String::new()
                }
            }
            Self::String(value) => value.clone(),
            Self::Void => String::from("void"),
        }
    }
}
