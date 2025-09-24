use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub import: ImportConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
pub struct ImportConfig {
    pub sort: bool,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self { sort: true }
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
