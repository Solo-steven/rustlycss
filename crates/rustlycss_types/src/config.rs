use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct GeneralConfig {
    pub minify: bool,
    pub sourcemap: bool
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneralConfig {
    pub fn new() -> Self {
        Self { minify: false, sourcemap: false }
    }
    pub fn from(minify: bool, sourcemap: bool) -> Self {
        Self { minify, sourcemap }
    }
}