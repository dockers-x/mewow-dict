use actix_web::{web, HttpResponse, Result};
use serde_derive::Deserialize;
use log::info;

use crate::lucky;
use crate::query::query;
use crate::mdd_manager::MDD_MANAGER;

#[derive(Deserialize, Debug)]
pub struct QueryForm {
    word: String,
}

pub(crate) async fn handle_query(params: web::Form<QueryForm>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{}", query(params.word.clone()))))
}

pub(crate) async fn handle_lucky() -> Result<HttpResponse> {
    let word = lucky::lucky_word();
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{}", query(word))))
}

/// Handle MDD resource requests
/// URL format: /mdd/{resource_path}
pub(crate) async fn handle_mdd_resource(path: web::Path<String>) -> Result<HttpResponse> {
    let resource_path = path.into_inner();
    info!("Requesting MDD resource: {}", resource_path);
    
    let manager = MDD_MANAGER.lock().unwrap();
    
    if let Some(data) = manager.get_resource(&resource_path) {
        // Determine content type based on file extension
        let content_type = get_content_type(&resource_path);
        
        Ok(HttpResponse::Ok()
            .content_type(content_type)
            .body(data))
    } else {
        info!("MDD resource not found: {}", resource_path);
        Ok(HttpResponse::NotFound().body("Resource not found"))
    }
}

/// Determine content type based on file extension
fn get_content_type(path: &str) -> &'static str {
    let path_lower = path.to_lowercase();
    
    if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if path_lower.ends_with(".png") {
        "image/png"
    } else if path_lower.ends_with(".gif") {
        "image/gif"
    } else if path_lower.ends_with(".svg") {
        "image/svg+xml"
    } else if path_lower.ends_with(".webp") {
        "image/webp"
    } else if path_lower.ends_with(".ico") {
        "image/x-icon"
    } else if path_lower.ends_with(".mp3") {
        "audio/mpeg"
    } else if path_lower.ends_with(".wav") {
        "audio/wav"
    } else if path_lower.ends_with(".ogg") {
        "audio/ogg"
    } else if path_lower.ends_with(".mp4") {
        "video/mp4"
    } else if path_lower.ends_with(".webm") {
        "video/webm"
    } else if path_lower.ends_with(".css") {
        "text/css"
    } else if path_lower.ends_with(".js") {
        "application/javascript"
    } else if path_lower.ends_with(".json") {
        "application/json"
    } else if path_lower.ends_with(".xml") {
        "application/xml"
    } else if path_lower.ends_with(".html") || path_lower.ends_with(".htm") {
        "text/html"
    } else if path_lower.ends_with(".txt") {
        "text/plain"
    } else if path_lower.ends_with(".woff") {
        "font/woff"
    } else if path_lower.ends_with(".woff2") {
        "font/woff2"
    } else if path_lower.ends_with(".ttf") {
        "font/ttf"
    } else if path_lower.ends_with(".otf") {
        "font/otf"
    } else {
        "application/octet-stream"
    }
}
