use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    name: String,
    version: Version,
}

impl Dependency {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_version(&self) -> Version {
        self.version.clone()
    }
}
