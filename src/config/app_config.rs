use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

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
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(path, config_str)?;
        Ok(())
    }

    // pub fn find_tunnel(&self, name: &str) -> Option<&TunnelConfig> {
    //     self.tunnels.iter().find(|t| t.name == name)
    // }
}
