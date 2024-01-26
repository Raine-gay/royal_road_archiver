use std::{collections::HashMap, process::exit};

use reqwest::{blocking::Response, header::HeaderMap};
use url::Url;

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
    pub fn get_text(self) -> String {
        match self.response.text() {
            Ok(response_text) => response_text,
            Err(error) => {
                eprintln!("Error! Unable to convert response from {0} into text\n{error}", self.url);
                exit(1);
            }
        }
    }

    /// Attempt to convert the response to bytes. Used for images. Exits the program if it fails.
    pub fn get_bytes(self) -> bytes::Bytes{
        match self.response.bytes() {
            Ok(response_bytes) => response_bytes,
            Err(error) => {
                eprintln!("Error! Unable to convert response from {0} into bytes\n{error}", self.url);
                exit(1);
            }
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
                eprintln!("Warning! Unable to get content type from the http-header: {warning}");
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
pub fn get_response(url: Url) -> HttpResponse {
    let response_result = reqwest::blocking::get(url.clone());

    match response_result {
        Ok(response) => HttpResponse { url, response },
        Err(error) => {
            eprintln!("Error! Unable to get a response from: {url}\n{error}");
            exit(1);
        },
    }
}

/// A function to convert a string to a url. Exits the program if it fails.
pub fn string_to_url(url: &str) -> Url {
    match Url::parse(url) {
        Ok(url) => url,
        Err(error) => {
            eprintln!("Error! Unable to parse: {url} into a valid url.\n{error}");
            exit(1);
        }
    }
}