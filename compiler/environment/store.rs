use crate::Object;
use sflynlang_parser::ast::DataType;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Store {
    data_types: HashMap<String, DataType>,
    objects: HashMap<String, Object>,

    outer: Option<Box<Store>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data_types: HashMap::new(),
            objects: HashMap::new(),

            outer: None,
        }
    }

    pub fn from_outer(outer: &Store) -> Self {
        let mut store = Self::new();

        store.outer = Some(Box::new(outer.clone()));

        store
    }

    pub fn get_data_types(&self) -> HashMap<String, DataType> {
        self.data_types.clone()
    }

    pub fn get_objects(&self) -> HashMap<String, Object> {
        self.objects.clone()
    }

    pub fn get_outer(&self) -> Option<Box<Store>> {
        self.outer.clone()
    }

    pub fn has_data_type(&self, key: &String) -> bool {
        self.data_types.contains_key(key)
    }

    pub fn has_data_type_with_outer(&self, key: &String) -> bool {
        self.has_data_type(key)
            || (self.get_outer().is_some()
                && self.get_outer().unwrap().has_data_type_with_outer(key))
    }

    pub fn has_object(&self, key: &String) -> bool {
        self.objects.contains_key(key)
    }

    pub fn has_object_with_outer(&self, key: &String) -> bool {
        self.has_object(key)
            || (self.get_outer().is_some()
                && self.get_outer().unwrap().has_object_with_outer(key))
    }

    pub fn is_builtin(&self, key: &String) -> bool {
        key == "print" || key == "debug"
    }

    pub fn has_key(&self, key: &String) -> bool {
        self.is_builtin(key)
            || self.has_data_type_with_outer(key)
            || self.has_object_with_outer(key)
    }

    pub fn add_data_type(&mut self, key: String, value: DataType) {
        self.data_types.insert(key, value);
    }

    pub fn add_object(&mut self, key: String, value: Object) {
        self.objects.insert(key, value);
    }
}
