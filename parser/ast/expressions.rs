use crate::{
    ast::{DataType, Node, Statement},
    Token,
};
use std::fmt;

pub type Expression = Node<Expressions>;

impl Expression {
    pub fn to_string(&self) -> String {
        self.node.to_string()
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Debug)]
pub enum Expressions {
    Argument {
        name: String,
        data_type: Box<DataType>,
        value: Option<Box<Expression>>,
    },

    Assignment {
        identifier: Box<Expression>,
        sign: Token,
        value: Box<Expression>,
    },

    Boolean(bool),

    Call(Box<Expression>, Vec<Expression>),

    Identifier(String),

    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Vec<Statement>,
    },

    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },

    Method(Box<Expression>, Box<Expression>),

    Number(f64),

    Prefix(Token, Box<Expression>),

    String(String),
}

impl Expressions {
    pub fn get_argument(
        &self,
    ) -> Option<(String, Box<DataType>, Option<Box<Expression>>)> {
        match self {
            Self::Argument {
                name,
                data_type,
                value,
            } => Some((name.clone(), data_type.clone(), value.clone())),
            _ => None,
        }
    }

    pub fn get_assignment(
        &self,
    ) -> Option<(Box<Expression>, Token, Box<Expression>)> {
        match self {
            Self::Assignment {
                identifier,
                sign,
                value,
            } => Some((identifier.clone(), sign.clone(), value.clone())),
            _ => None,
        }
    }

    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_call(&self) -> Option<(Box<Expression>, Vec<Expression>)> {
        match self {
            Self::Call(identifier, arguments) => {
                Some((identifier.clone(), arguments.clone()))
            }
            _ => None,
        }
    }

    pub fn get_identifier(&self) -> Option<String> {
        match self {
            Self::Identifier(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_if(
        &self,
    ) -> Option<(Box<Expression>, Vec<Statement>, Vec<Statement>)> {
        match self {
            Self::If {
                condition,
                consequence,
                alternative,
            } => Some((
                condition.clone(),
                consequence.clone(),
                alternative.clone(),
            )),
            _ => None,
        }
    }

    pub fn get_infix(
        &self,
    ) -> Option<(Box<Expression>, Token, Box<Expression>)> {
        match self {
            Self::Infix {
                left,
                operator,
                right,
            } => Some((left.clone(), operator.clone(), right.clone())),
            _ => None,
        }
    }

    pub fn get_method(&self) -> Option<(Box<Expression>, Box<Expression>)> {
        match self {
            Self::Method(identifier, property) => {
                Some((identifier.clone(), property.clone()))
            }
            _ => None,
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_prefix(&self) -> Option<(Token, Box<Expression>)> {
        match self {
            Self::Prefix(operator, value) => {
                Some((operator.clone(), value.clone()))
            }
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
            Self::Argument {
                name,
                data_type,
                value,
            } => format!(
                "{}: {}{}",
                name,
                data_type,
                match value {
                    Some(value) => format!(" = {}", value),
                    None => String::new(),
                },
            ),
            Self::Assignment {
                identifier,
                sign,
                value,
            } => format!("{} {} {};", identifier, sign, value,),
            Self::Boolean(_) => String::from("Boolean"),
            Self::Call(identifier, arguments) => format!(
                "{}({})",
                identifier,
                arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Identifier(value) => value.clone(),
            Self::If {
                condition,
                consequence,
                alternative,
            } => format!(
                "if ({}) {{\n{}\n}}{}",
                condition,
                consequence
                    .iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
                if alternative.len() > 0 {
                    format!(
                        " else {{\n{}\n}}",
                        alternative
                            .iter()
                            .map(|stmt| stmt.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    )
                } else {
                    String::new()
                },
            ),
            Self::Infix {
                left,
                operator,
                right,
            } => format!("{} {} {}", left, operator, right),
            Self::Method(identifier, property) => {
                format!("{}.{}", identifier, property)
            }
            Self::Number(_) => String::from("Number"),
            Self::Prefix(operator, value) => format!("{}{}", operator, value),
            Self::String(_) => String::from("String"),
        }
    }
}
