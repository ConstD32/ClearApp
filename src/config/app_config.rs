use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::services::logging::log_message;

pub const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub folders: Vec<ConfigFolders>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigFolders {
    pub name: String,
    pub path: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_file(CONFIG_FILE)
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();

        match fs::read_to_string(path) {
            Ok(config_str) => {
                let config: AppConfig = serde_json::from_str(&config_str)?;
                Ok(config)
            }

            Err(_) => {
                // файл не существует или не читается → создаём дефолт
                Self::default_and_save(path)
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to_file(CONFIG_FILE)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();

        // гарантируем существование каталога
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(path, config_str)?;

        Ok(())
    }

    pub fn default_and_save<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let default_config = AppConfig {
            folders: vec![ConfigFolders { name: "Disk C:/Temp".to_string(), path: "C:/Temp".to_string() }],
        };

        default_config.save_to_file(&path)?;
        Ok(default_config)
    }
}

pub(crate) fn update() {
    log_message("Проверка обновлений...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("ConstD32")
        .repo_name("ClearApp")
        .bin_name("ClearApp")
        .show_download_progress(false) // ← ключевое
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()
        .and_then(|u| u.update());

    match status {
        Ok(s) => {
            log_message(&format!("Обновлено до v{}", s.version()));
        }
        Err(e) => {
            log_message(&format!("Ошибка обновления: {}", e));
        }
    }
}
