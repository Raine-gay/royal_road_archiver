use std::collections::HashMap;

use reqwest::{blocking::Response, header::HeaderMap};
use url::Url;

use crate::{GenerationError, Warning, WARNINGS};

// A struct representing an HttpResponse and the Url it originated from.
pub struct HttpResponse {
    url: Url,
    pub response: Response,
}

impl HttpResponse {
    /// Get the response headers.
    pub fn get_headers(&self) -> &HeaderMap {
        self.response.headers()
    }

    /// Attempt to convert the response to text. Exits the program if it fails.
    pub fn get_text(self) -> Result<String, GenerationError> {
        match self.response.text() {
            Ok(response_text) => Ok(response_text),
            Err(error) => Err(GenerationError::ResponseConvertToTextError {error}),
        }
    }

    /// Attempt to convert the response to bytes. Used for images. Exits the program if it fails.
    pub fn get_bytes(self) -> Result<bytes::Bytes, GenerationError>{
        match self.response.bytes() {
            Ok(response_bytes) => Ok(response_bytes),
            Err(error) => Err(GenerationError::ResponseConvertToBytesError {error}),
        }
    }

    /// Attempt to get the content(mime)-type and file extension from the http-header.
    /// 
    /// If the content-type header value can not be found it will warn the use and return empty strings.
    pub fn get_content_type_and_file_extension(&self) -> (String, String) {
        // A hashmap to convert mime-types to file extensions.
        let mime_to_file_extension: HashMap<&str, &str> = HashMap::from([
            ("image/png",  "png"),
            ("image/webp", "webp"),
            ("image/jpeg", "jpeg"),
            ("image/jpg",  "jpg"),
        ]);

        let content_type = match self.get_headers()["content-type"].to_str() {
            Ok(content_type) => content_type,
            Err(warning) => {
                let warning = Warning::MissingContentType { 
                    warning_msg: "Unable to find or parse the content-type header".to_string(), 
                    url: self.url.clone(),
                    error: warning,
                };
                WARNINGS.lock().unwrap().add_warning(warning);
                
                return (String::with_capacity(0), String::with_capacity(0));
            }
        };

        if mime_to_file_extension.contains_key(content_type) {
            return (content_type.to_string(), mime_to_file_extension[content_type].to_string());
        }
        else {
            return (content_type.to_string(), String::with_capacity(0));
        }
    }
}

/// Get an http response for a given url. Exits the program if it fails.
pub fn get_response(url: Url) -> Result<HttpResponse, GenerationError> {
    let response_result = reqwest::blocking::get(url.clone());

    match response_result {
        Ok(response) => Ok(HttpResponse { url, response }),
        Err(error) => return Err(GenerationError::ResponseGetError {error, url}),
    }
}

/// A function to convert a string to a url. Exits the program if it fails.
pub fn string_to_url(url: &str) -> Result<Url, GenerationError> {
    match Url::parse(url) {
        Ok(url) => Ok(url),
        Err(error) => Err(GenerationError::UrlParseError {error, string_url: url.to_string()}),
    }
}