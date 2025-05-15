use log::{info, error};
use rusqlite::{named_params, Connection};
use std::fs;
use std::collections::HashSet;

use crate::config::Config;

pub fn query(word: String) -> String {
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load config: {}", e);
            return "Error: Failed to load configuration".to_string();
        }
    };
    
    let w = word;
    let mut htmls = Vec::new();
    let mut seen_defs = HashSet::new();
    for dir in config.get_all_dict_dirs() {
        let dir_path = dir.clone();
        let entries = match fs::read_dir(&dir_path) {
            Ok(entries) => entries,
            Err(e) => {
                error!("Failed to read directory {:?}: {}", dir_path, e);
                continue;
            }
        };

        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "mdx" {
                    let pathbuf = entry.path();
                    let path = match pathbuf.to_str() {
                        Some(path) => path.to_string(),
                        None => continue,
                    };
                    let dict_name = pathbuf
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("未知词典")
                        .to_string();
                    let db_file = format!("{}{}", path, ".db");
                    let conn = match Connection::open(&db_file) {
                        Ok(conn) => conn,
                        Err(e) => {
                            error!("Failed to open database {}: {}", db_file, e);
                            continue;
                        }
                    };

                    let mut stmt = match conn.prepare("select * from MDX_INDEX WHERE text= :word limit 1;") {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            error!("Failed to prepare statement: {}", e);
                            continue;
                        }
                    };

                    info!("query params={}", &w);

                    let mut rows = match stmt.query(named_params! { ":word": w.clone() }) {
                        Ok(rows) => rows,
                        Err(e) => {
                            error!("Failed to execute query: {}", e);
                            continue;
                        }
                    };

                    if let Some(row) = rows.next().unwrap_or(None) {
                        match row.get::<usize, String>(1) {
                            Ok(def) => {
                                if seen_defs.insert(def.clone()) {
                                    htmls.push(format!(
                                        "<div class=\"dict-block\"><div class=\"dict-source\">{}</div><div class=\"dict-content\">{}</div></div>",
                                        dict_name, def
                                    ));
                                }
                            },
                            Err(e) => {
                                error!("Failed to get definition: {}", e);
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
    if htmls.is_empty() {
        "not found".to_string()
    } else {
        htmls.join("")
    }
}
