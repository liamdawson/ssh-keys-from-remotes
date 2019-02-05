use config::{Config, ConfigError, File, FileFormat};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SettingsUser {
    pub urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub users: HashMap<String, SettingsUser>,
}

impl Settings {
    pub fn new(path: PathBuf) -> Result<Settings, ConfigError> {
        let mut config = Config::new();
        config.merge(File::from(path).format(FileFormat::Toml))?;

        config.try_into()
    }
}
