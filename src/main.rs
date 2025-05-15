use std::error::Error;
use std::env;
use std::path::PathBuf;

use actix_files;
use actix_web::{middleware, web, App, HttpServer, HttpResponse, HttpRequest};
use pretty_env_logger;

use crate::config::{ Config, get_embedded_file};
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
            .service(
                web::resource("/{tail:.*}").route(web::get().to(static_handler))
            ),
    );
}

async fn static_handler(req: HttpRequest) -> HttpResponse {
    let path = req.path();
    let normalized_path = if path == "/" {
        "/index.html"
    } else {
        path
    };

    println!("Requested path: {}", normalized_path);

    // 只允许用户提供 CSS 文件
    if normalized_path.ends_with(".css") {
        if let Some(user_dir) = crate::config::user_static_path() {
            let file_path = user_dir.join(&normalized_path[1..]); // 去掉开头的'/'
            println!("Looking for user CSS at: {:?}", file_path);
            if file_path.exists() && file_path.is_file() {
                if let Ok(content) = std::fs::read(&file_path) {
                    println!("Found user CSS file");
                    return HttpResponse::Ok()
                        .content_type("text/css")
                        .body(content);
                }
            }
        }
    }

    // 其他所有资源都使用内置的
    if let Some(content) = crate::config::get_embedded_file(normalized_path) {
        println!("Serving embedded file: {}", normalized_path);
        let content_type = if normalized_path.ends_with(".css") {
            "text/css"
        } else if normalized_path.ends_with(".html") {
            "text/html"
        } else if normalized_path.ends_with(".ico") {
            "image/x-icon"
        } else if normalized_path.ends_with(".js") {
            "application/javascript"
        } else {
            "application/octet-stream"
        };
        HttpResponse::Ok().content_type(content_type).body(content)
    } else {
        println!("404 Not Found: {}", normalized_path);
        HttpResponse::NotFound().finish()
    }
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
