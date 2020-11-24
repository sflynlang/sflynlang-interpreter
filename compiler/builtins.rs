use crate::{Object, Objects};
use sflynlang_parser::{
    ast::{DataType, DataTypes},
    Error, Position,
};

pub fn print(
    objects: Vec<Object>,
    position: Position,
) -> Result<Object, Error> {
    if objects.len() != 1 {
        return Err(Error::new_expect_arguments(position, 1, objects.len()));
    }

    let object = objects[0].clone();

    if object.get_node().get_string().is_none() {
        return Err(Error::new_expect_type(
            object.get_position(),
            "String",
            &object.to_data_type().to_string(),
        ));
    }

    println!("{}", object.to_string());

    Ok(Object::new(object.get_position(), Objects::Void))
}

pub fn debug(
    objects: Vec<Object>,
    position: Position,
    debug_mode: bool,
) -> Result<Object, Error> {
    if objects.len() != 1 {
        return Err(Error::new_expect_arguments(position, 1, objects.len()));
    }

    let object = objects[0].clone();

    if object.get_node().get_string().is_none() {
        return Err(Error::new_expect_type(
            object.get_position(),
            "String",
            &object.to_data_type().to_string(),
        ));
    }

    if debug_mode {
        println!("{}", object.to_string());
    }

    Ok(Object::new(object.get_position(), Objects::Void))
}

pub fn check_builtin(
    key: String,
    position: Position,
    arguments: Vec<DataType>,
) -> Result<DataType, Error> {
    match key.as_str() {
        "print" | "debug" => {
            if arguments.len() != 1 {
                return Err(Error::new_expect_arguments(
                    position,
                    1,
                    arguments.len(),
                ));
            }

            if arguments[0].node == DataTypes::String {
                Ok(DataType::new(position, DataTypes::Void))
            } else {
                Err(Error::new_expect_type(
                    position,
                    "string",
                    &arguments[0].node.to_string(),
                ))
            }
        }

        _ => Err(Error::new_unknown_identifier(position, key)),
    }
}
