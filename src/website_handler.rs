use crate::http::request;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
use std::path::PathBuf;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<(Vec<u8>, String)> {
        // Build secure full path
        let mut full_path = PathBuf::from(&self.public_path);
        let clean_path = file_path.trim_start_matches('/');
        full_path.push(clean_path);

        // Prevent directory traversal
        if !full_path.starts_with(&self.public_path) {
            println!("Directory traversal attempt detected: {}", file_path);
            return None;
        }

        // Read raw bytes (works for text AND binary)
        let content = match fs::read(&full_path) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };

        // ── Use infer to detect real MIME type from content ──
        let mime_type = infer::get(&content)
            .map(|kind| kind.mime_type().to_string())
            .unwrap_or_else(|| self.fallback_mime_from_extension(&full_path));

        Some((content, mime_type))
    }

    fn fallback_mime_from_extension(&self, path: &PathBuf) -> String {
        match path.extension().and_then(|e| e.to_str()) {
            Some("html") | Some("htm") => "text/html; charset=utf-8".to_string(),
            Some("css") => "text/css; charset=utf-8".to_string(),
            Some("js") => "application/javascript; charset=utf-8".to_string(),
            Some("json") => "application/json; charset=utf-8".to_string(),
            Some("txt") => "text/plain; charset=utf-8".to_string(),
            Some("png") => "image/png".to_string(),
            Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
            Some("gif") => "image/gif".to_string(),
            Some("webp") => "image/webp".to_string(),
            Some("svg") => "image/svg+xml".to_string(),
            Some("ico") => "image/x-icon".to_string(),
            Some("woff2") => "font/woff2".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => {
                let path = request.path();

                // Root should serve index.html
                let target_path = if path == "/" { "index.html" } else { path };

                match self.read_file(target_path) {
                    Some((content, mime)) => {
                        // Success → send bytes + detected Content-Type
                        Response::new(StatusCode::Ok, Some(content), Some(mime))
                    }
                    None => self.not_found(),
                }
            }
            _ => Response::new(StatusCode::NotFound, None, None),
        }
    }
}

impl WebsiteHandler {
    fn not_found(&self) -> Response {
        let body = b"<!DOCTYPE html><html><head><title>404</title></head><body><h1>404 - Not Found</h1><p>Resource not found.</p></body></html>";
        Response::new(
            StatusCode::NotFound,
            Some(body.to_vec()),
            Some("text/html; charset=utf-8".to_string()),
        )
    }
}
