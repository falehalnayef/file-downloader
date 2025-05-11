use url::Url;

use reqwest::blocking::Client;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use std::time::Duration;

pub fn validate_url(url_str: &str) -> Result<Url, String> {
    let url = Url::parse(url_str).map_err(|e| format!("Invalid URL format: {}", e))?;

    match url.scheme() {
        "http" | "https" => Ok(url),
        _ => Err("Only HTTP and HTTPS are supported".to_string()),
    }
}

pub fn is_valid_file_url(url: &str) -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    let response = client
        .head(url)
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Server returned error: {}", response.status()));
    }

    let headers = response.headers();

    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if content_type.starts_with("text/html") || content_type.starts_with("application/xhtml+xml") {
        return Err("URL points to an HTML page, not a file.".to_string());
    }

    if headers.get(CONTENT_LENGTH).is_none() {
        return Err("Missing Content-Length, can't determine file size.".to_string());
    }

    Ok(())
}
