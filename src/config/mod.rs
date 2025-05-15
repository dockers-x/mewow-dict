use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub builtin_dict_dirs: Vec<PathBuf>,
    pub user_dict_dirs: Vec<PathBuf>,
}

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

impl Default for Config {
    fn default() -> Self {
        let mut builtin_dict_dirs = Vec::new();
        let mut user_dict_dirs: Vec<PathBuf> = Vec::new();

        // 首先检查 BUILTIN_DICT_DIR
        if let Ok(builtin_dict_dir) = env::var("BUILTIN_DICT_DIR") {
            println!("BUILTIN_DICT_DIR env var: {}", builtin_dict_dir);
            let path = expand_tilde(&builtin_dict_dir);
            println!("Expanded builtin dict dir: {:?}", path);
            if path.exists() {
                println!("Builtin dict dir exists");
                if let Ok(entries) = std::fs::read_dir(&path) {
                    let mdx_files: Vec<_> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().extension().map_or(false, |ext| ext == "mdx"))
                        .collect();
                    println!("Found {} MDX files in builtin dict dir", mdx_files.len());
                }
                builtin_dict_dirs.push(path);
            } else {
                println!("Builtin dict dir does not exist");
            }
        } else {
            println!("BUILTIN_DICT_DIR not set");
        }

        // 然后检查 CARGO_MANIFEST_DIR
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let manifest_dir = manifest_dir.clone();
            let mut path: PathBuf = manifest_dir.clone().into();
            path.push("resources/mdx");
            println!("Checking manifest mdx dir: {:?}", path);
            if path.exists() {
                println!("Found manifest mdx dir");
                builtin_dict_dirs.push(path);
            }
            
            // Add resources/user_mdx if it exists
            let mut user_mdx_path: PathBuf = manifest_dir.into();
            user_mdx_path.push("resources/user_mdx");
            println!("Checking user mdx dir: {:?}", user_mdx_path);
            if user_mdx_path.exists() {
                println!("Found user mdx dir");
                user_dict_dirs.push(user_mdx_path);
            }
        }

        // Add USER_DICT_DIR if it exists
        if let Ok(user_dict_dir) = env::var("USER_DICT_DIR") {
            println!("USER_DICT_DIR env var: {}", user_dict_dir);
            let path = expand_tilde(&user_dict_dir);
            println!("Expanded user dict dir: {:?}", path);
            if path.exists() {
                println!("User dict dir exists");
                if let Ok(entries) = std::fs::read_dir(&path) {
                    let mdx_files: Vec<_> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().extension().map_or(false, |ext| ext == "mdx"))
                        .collect();
                    println!("Found {} MDX files in user dict dir", mdx_files.len());
                }
                user_dict_dirs.push(path);
            } else {
                println!("User dict dir does not exist");
            }
        } else {
            println!("USER_DICT_DIR not set");
        }

        println!("Final builtin dict dirs: {:?}", builtin_dict_dirs);
        println!("Final user dict dirs: {:?}", user_dict_dirs);
        
        Self {
            builtin_dict_dirs,
            user_dict_dirs,
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
        path.push("mewow-dict");
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

pub fn get_embedded_file(path: &str) -> Option<&'static [u8]> {
    match path {
        "/" | "/index.html" => Some(include_bytes!("../../assets/web/index.html")),
        "/index.js" => Some(include_bytes!("../../assets/web/index.js")),
        "/index.css" => Some(include_bytes!("../../assets/web/index.css")),
        "/favicon.ico" => Some(include_bytes!("../../assets/web/favicon.ico")),
        "/jquery.min.js" => Some(include_bytes!("../../assets/web/jquery.min.js")),
        "/hycd_3rd.js" => Some(include_bytes!("../../assets/web/hycd_3rd.js")),
        "/LSC4.css" => Some(include_bytes!("../../assets/web/LSC4.css")),
        "/O8C.css" => Some(include_bytes!("../../assets/web/O8C.css")),
        "/hycd_3rd.css" => Some(include_bytes!("../../assets/web/hycd_3rd.css")),
        "/hycd_3rd_img.css" => Some(include_bytes!("../../assets/web/hycd_3rd_img.css")),
        "/hycd_3rd_img.css" => Some(include_bytes!("../../assets/web/hycd_3rd_img.css")),
        "/sound.png" => Some(include_bytes!("../../assets/web/sound.png")),
        _ => {
            println!("No embedded file found for path: {}", path);
            None
        }
    }
}

pub fn user_static_path() -> Option<PathBuf> {
    env::var("USER_STATIC_PATH").ok().map(PathBuf::from)
}
