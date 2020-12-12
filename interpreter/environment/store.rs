use crate::Object;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Store {
    objects: HashMap<String, Object>,

    outer: Option<Box<Store>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),

            outer: None,
        }
    }

    pub fn from_outer(outer: &Store) -> Self {
        let mut store = Self::new();

        store.outer = Some(Box::new(outer.clone()));

        store
    }

    pub fn get_objects(&self) -> HashMap<String, Object> {
        self.objects.clone()
    }

    pub fn get_outer(&self) -> Option<Box<Store>> {
        self.outer.clone()
    }

    pub fn has_object(&self, key: &String) -> bool {
        self.objects.contains_key(key)
    }

    pub fn has_object_with_outer(&self, key: &String) -> bool {
        self.has_object(key)
            || (self.get_outer().is_some()
                && self.get_outer().unwrap().has_object_with_outer(&key))
    }

    pub fn has_key(&self, key: &String) -> bool {
        if key == "print" || key == "debug" {
            return true;
        }

        self.has_object_with_outer(key)
    }
}
