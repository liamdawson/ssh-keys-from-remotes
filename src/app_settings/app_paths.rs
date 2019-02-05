const CONFIG_ROOT: &str = "/etc";
const CACHE_ROOT: &str = "/var/cache";
const CONFIG_FILE_NAME: &str = "ssh-keys-remotes.toml";
const CACHE_DIR_NAME: &str = "ssh-keys-from-remotes";

use std::path::PathBuf;

pub fn default_config_file_path() -> PathBuf {
    let root = option_env!("SKFR_CONFIG_ROOT").unwrap_or(CONFIG_ROOT);

    PathBuf::from(format!("{}/{}", root, CONFIG_FILE_NAME))
}

pub fn default_cache_path() -> PathBuf {
    let root = option_env!("SKFR_CACHE_ROOT").unwrap_or(CACHE_ROOT);

    PathBuf::from(format!("{}/{}/", root, CACHE_DIR_NAME))
}
