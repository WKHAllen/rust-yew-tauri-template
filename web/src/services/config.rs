use super::super::types::AppConfig;
use super::local_storage;

pub fn load_config() -> AppConfig {
    AppConfig {
        message: local_storage::get_item("message").unwrap_or(String::from("Hello, world!")),
    }
}

pub fn save_config(config: &AppConfig) {
    local_storage::set_item("message", &config.message);
}
