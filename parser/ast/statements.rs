use crate::ast::{DataType, Expression, Node};
use std::fmt;

pub type Statement = Node<Statements>;

impl Statement {
    pub fn to_string(&self) -> String {
        self.node.to_string()
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Debug)]
pub enum Statements {
    Expression(Box<Expression>),

    Function {
        name: Box<Expression>,
        arguments: Vec<Expression>,
        return_type: Box<DataType>,
        body: Vec<Statement>,
    },

    Return(Option<Expression>),

    Variable {
        is_mutable: bool,
        name: String,
        data_type: Option<DataType>,
        value: Option<Expression>,
    },
}

impl Statements {
    pub fn get_expression(&self) -> Option<Box<Expression>> {
        match self {
            Self::Expression(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_function(
        &self,
    ) -> Option<(
        Box<Expression>,
        Vec<Expression>,
        Box<DataType>,
        Vec<Statement>,
    )> {
        match self {
            Self::Function {
                name,
                arguments,
                return_type,
                body,
            } => Some((
                name.clone(),
                arguments.clone(),
                return_type.clone(),
                body.clone(),
            )),
            _ => None,
        }
    }

    pub fn get_return(&self) -> Option<Option<Expression>> {
        match self {
            Self::Return(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_variable(&self) -> Option<(bool, String, Option<DataType>, Option<Expression>)> {
        match self {
            Self::Variable {
                is_mutable,
                name,
                data_type,
                value,
            } => Some((
                is_mutable.clone(),
                name.clone(),
                data_type.clone(),
                value.clone(),
            )),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Expression(value) => value.to_string(),
            Self::Function {
                name,
                arguments,
                return_type,
                body,
            } => format!(
                "func {} ({}): {} {{\n{}\n}}",
                name,
                arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                return_type,
                body.iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
            Self::Return(value) => format!(
                "return{};",
                match value {
                    Some(value) => format!(" {}", value),
                    None => String::new(),
                },
            ),
            Self::Variable {
                is_mutable,
                name,
                data_type,
                value,
            } => format!(
                "{} {}{}{};",
                if *is_mutable { "let" } else { "const" },
                name,
                match data_type {
                    Some(data_type) => format!(": {}", data_type),
                    None => String::new(),
                },
                match value {
                    Some(value) => format!(" = {}", value),
                    None => String::new(),
                },
            ),
        }
    }
}
