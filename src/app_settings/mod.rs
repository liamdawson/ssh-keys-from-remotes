mod app_paths;
mod settings;

use app_paths::default_cache_path;
use config::ConfigError;
use settings::Settings;
use std::path::PathBuf;

pub fn config_from_default_location() -> Result<Settings, ConfigError> {
    Settings::new(app_paths::default_config_file_path())
}

pub fn default_cache_location() -> std::io::Result<PathBuf> {
    let default_path = default_cache_path();

    std::fs::create_dir_all(&default_path).map(|()| default_path)
}
