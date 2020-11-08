use crate::structures::SlangDependency;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Slang {
    name: String,
    main: String,
    version: Version,
    authors: Option<Vec<String>>,
    dependencies: Option<Vec<SlangDependency>>,
}

impl Slang {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            main: String::from("index.sf"),
            version: Version::new(0, 1, 0),
            authors: None,
            dependencies: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_main(&self) -> String {
        self.main.clone()
    }

    pub fn set_main(&mut self, main: String) {
        self.main = main;
    }

    pub fn get_version(&self) -> Version {
        self.version.clone()
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn get_authors(&self) -> Option<Vec<String>> {
        self.authors.clone()
    }

    pub fn has_author(&self, author_name: &String) -> bool {
        if let Some(authors) = self.get_authors() {
            authors.contains(author_name)
        } else {
            false
        }
    }

    pub fn add_author(&mut self, author_name: String) {
        if self.get_authors() == None {
            self.authors = Some(Vec::new());
        }

        if !self.has_author(&author_name) {
            let mut authors = self.get_authors().unwrap();

            authors.push(author_name);

            self.authors = Some(authors);
        }
    }

    pub fn get_dependencies(&self) -> Option<Vec<SlangDependency>> {
        self.dependencies.clone()
    }

    pub fn has_dependency(&self, dependency_name: &String) -> bool {
        if let Some(dependencies) = self.get_dependencies() {
            for dependency in dependencies.iter() {
                if &dependency.get_name() == dependency_name {
                    return true;
                }
            }
        }

        false
    }
}
