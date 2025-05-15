use std::env;
use std::path::PathBuf;
use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub builtin_dict_dirs: Vec<PathBuf>,
    pub user_dict_dirs: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        let mut builtin_dict_dirs = Vec::new();
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path: PathBuf = manifest_dir.into();
            path.push("resources/mdx");
            builtin_dict_dirs.push(path.clone());
            builtin_dict_dirs.push(path);
        }
        
        Self {
            builtin_dict_dirs,
            user_dict_dirs: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let mut path = Self::user_config_dir()?;
        path.push("mdict-rs");
        fs::create_dir_all(&path)?;
        path.push("config.json");
        Ok(path)
    }

    pub fn user_config_dir() -> Result<PathBuf> {
        let mut path = if let Some(home) = dirs::home_dir() {
            home
        } else {
            env::current_dir()?
        };
        path.push(".config");
        Ok(path)
    }

    pub fn get_all_dict_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = self.builtin_dict_dirs.clone();
        dirs.extend(self.user_dict_dirs.clone());
        dirs
    }
}

pub fn static_path() -> Result<PathBuf> {
    let mut path: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    path.push("resources/static");
    Ok(path)
}
