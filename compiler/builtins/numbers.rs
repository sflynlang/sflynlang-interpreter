use crate::{Environment, Object, Objects};
use sflynlang_parser::{
    ast::{DataType, DataTypes},
    Error, Position,
};
use std::collections::HashMap;

pub fn get_hashmap_typechecker() -> HashMap<String, DataType> {
    let mut data: HashMap<String, DataType> = HashMap::new();

    data.insert(
        String::from("toString"),
        DataType::new(
            Position::new(0, 0, 1, 1),
            DataTypes::Function(
                Vec::new(),
                Box::new(DataType::new(
                    Position::new(0, 0, 1, 1),
                    DataTypes::String,
                )),
            ),
        ),
    );

    data
}

pub fn get_hashmap_compiler(
    object: Object,
    environment: Environment,
) -> Result<HashMap<String, Object>, Error> {
    if let Some(number) = object.get_node().get_number() {
        let mut data: HashMap<String, Object> = HashMap::new();

        data.insert(
            String::from("toString"),
            Object::new(
                Position::new(0, 0, 1, 1),
                Objects::Function {
                    arguments: HashMap::new(),
                    body: Vec::new(),
                    return_obj: Box::new(Object::new(
                        object.get_position(),
                        Objects::String(number.to_string()),
                    )),
                    environment,
                },
            ),
        );

        return Ok(data);
    }

    Err(Error::new_expect_type(
        object.get_position(),
        "Number",
        &object.to_string(),
    ))
}
