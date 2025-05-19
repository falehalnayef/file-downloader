use url::Url;

use reqwest::blocking::Client;
use reqwest::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE};
use std::time::Duration;

pub struct FileInfo {
    pub content_length: u64,
    pub file_name: String,
    pub supports_ranges: bool,
}
pub fn validate_url(url_str: &str) -> Result<Url, String> {
    let url = Url::parse(url_str).map_err(|e| format!("Invalid URL format: {}", e))?;

    match url.scheme() {
        "http" | "https" => Ok(url),
        _ => Err("Only HTTP and HTTPS are supported".to_string()),
    }
}

pub fn get_file_info(url: &str) -> Result<FileInfo, String> {
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

    let content_length = headers
        .get(CONTENT_LENGTH)
        .ok_or("Missing Content-Length header")?
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let file_name = if let Some(disposition) = response.headers().get(CONTENT_DISPOSITION) {
        let dispo_str = disposition;

        extract_filename_from_disposition(dispo_str.to_str().unwrap())
            .unwrap_or_else(|| "downloaded_file".to_string())
    } else {
        "downloaded_file".to_string()
    };
    let supports_ranges = response
        .headers()
        .get(ACCEPT_RANGES)
        .map_or(false, |v| v == "bytes");

    Ok(FileInfo {
        content_length,
        file_name,
        supports_ranges,
    })
}
fn extract_filename_from_disposition(dispo: &str) -> Option<String> {
    dispo.split(';').find_map(|part| {
        let part = part.trim();
        if part.starts_with("filename=") {
            Some(
                part.trim_start_matches("filename=")
                    .trim_matches('"')
                    .to_string(),
            )
        } else {
            None
        }
    })
}

pub fn calculate_ranges() {}
