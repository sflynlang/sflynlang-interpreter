mod store;

pub use store::Store;

use sflynlang_parser::{Error, File};

pub struct Environment {
    debug_mode: bool,
    store: Store,
    errors: Vec<Error>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            debug_mode: false,
            store: Store::new(),
            errors: Vec::new(),
        }
    }

    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }

    pub fn set_debug_mode(&mut self, debug_mode: bool) {
        self.debug_mode = debug_mode;
    }

    pub fn get_store(&self) -> Store {
        self.store.clone()
    }

    pub fn get_errors(&self) -> Vec<Error> {
        self.errors.clone()
    }

    pub fn has_errors(&self) -> bool {
        self.get_errors().len() > 0
    }

    pub fn show_errors(&self, file: &File) {
        if self.has_errors() {
            for error in self.get_errors().iter() {
                error.show(file);
            }
        }
    }

    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error)
    }
}
