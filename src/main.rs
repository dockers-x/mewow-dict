use std::error::Error;
use std::env;
use std::path::PathBuf;

use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use pretty_env_logger;

use crate::config::{static_path, Config};
use crate::handlers::{handle_lucky, handle_query};
use crate::indexing::indexing;

mod config;
mod handlers;
mod indexing;
mod lucky;
mod mdict;
mod query;
mod util;

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/query").route(web::post().to(handle_query)))
            .service(web::resource("/lucky").route(web::get().to(handle_lucky)))
            // .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "max-age=86400")))
            .service(
                actix_files::Files::new("/", static_path().unwrap().to_str().unwrap())
                    .index_file("index.html"),
            ), // static file 必须放在最后，否则会屏蔽其他route
    );
}

fn get_dict_files() -> Vec<String> {
    let config = Config::load().unwrap_or_default();
    let mut files = Vec::new();
    
    for dir in config.get_all_dict_dirs() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "mdx" {
                        if let Some(path) = entry.path().to_str() {
                            files.push(path.to_string());
                        }
                    }
                }
            }
        }
    }
    
    files
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let dict_files = get_dict_files();
    if dict_files.is_empty() {
        println!("Warning: No MDX files found in configured directories");
    } else {
        println!("Found {} MDX files", dict_files.len());
        indexing(&dict_files, false);
    }

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8181".to_string())
        .parse::<u16>()
        .unwrap_or(8181);

    println!("App serving on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind((host, port))?
    .run()
    .await
    .map_err(|e| Box::new(e) as Box<dyn Error>)
}
