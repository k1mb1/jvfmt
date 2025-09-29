use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
#[serde(default)]
pub struct Config {
    pub import: ImportConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default)]
pub struct ImportConfig {
    pub sort: bool,
    pub grouped: bool,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            sort: true,
            grouped: true,
        }
    }
}
pub trait ImportConfigProvider {
    fn import_config(&self) -> &ImportConfig;
}

impl ImportConfigProvider for Config {
    fn import_config(&self) -> &ImportConfig {
        &self.import
    }
}
