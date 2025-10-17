use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use log::{info, error};

use crate::mdict::mdd::Mdd;
use crate::config::Config;

/// Global MDD resource manager
/// Stores loaded MDD files in memory for fast access
pub static MDD_MANAGER: Lazy<Mutex<MddManager>> = Lazy::new(|| {
    Mutex::new(MddManager::new())
});

pub struct MddManager {
    mdds: HashMap<String, Mdd>,
}

impl MddManager {
    pub fn new() -> Self {
        MddManager {
            mdds: HashMap::new(),
        }
    }

    /// Load all MDD files from configured directories
    pub fn load_all_mdds(&mut self) {
        let config = match Config::load() {
            Ok(config) => config,
            Err(e) => {
                error!("Failed to load config: {}", e);
                return;
            }
        };

        for dir in config.get_all_dict_dirs() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "mdd" {
                            let path = entry.path();
                            if let Some(path_str) = path.to_str() {
                                info!("Loading MDD file: {}", path_str);
                                match fs::read(&path) {
                                    Ok(data) => {
                                        match std::panic::catch_unwind(|| Mdd::new(&data)) {
                                            Ok(mdd) => {
                                                let dict_name = path
                                                    .file_stem()
                                                    .and_then(|s| s.to_str())
                                                    .unwrap_or("unknown")
                                                    .to_string();
                                                self.mdds.insert(dict_name.clone(), mdd);
                                                info!("Successfully loaded MDD: {}", dict_name);
                                            }
                                            Err(e) => {
                                                error!("Failed to parse MDD file {}: {:?}", path_str, e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to read MDD file {}: {}", path_str, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        info!("Total MDD files loaded: {}", self.mdds.len());
    }

    /// Get a resource from any loaded MDD file by path
    pub fn get_resource(&self, resource_path: &str) -> Option<Vec<u8>> {
        // Try to find the resource in all loaded MDD files
        for mdd in self.mdds.values() {
            if let Some(data) = mdd.get_resource_by_path(resource_path) {
                return Some(data);
            }
        }
        None
    }

    /// Get a resource from a specific MDD file
    #[allow(dead_code)]
    pub fn get_resource_from_dict(&self, dict_name: &str, resource_path: &str) -> Option<Vec<u8>> {
        if let Some(mdd) = self.mdds.get(dict_name) {
            mdd.get_resource_by_path(resource_path)
        } else {
            None
        }
    }

    /// List all available dictionaries with MDD resources
    #[allow(dead_code)]
    pub fn list_dicts(&self) -> Vec<String> {
        self.mdds.keys().cloned().collect()
    }
}

/// Initialize the MDD manager by loading all MDD files
pub fn init_mdd_manager() {
    let mut manager = MDD_MANAGER.lock().unwrap();
    manager.load_all_mdds();
}
