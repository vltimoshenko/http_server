extern crate percent_encoding;

use std::clone::Clone;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use chrono::prelude::*;

use super::server::Response;

use self::percent_encoding::percent_decode;

const STATUS_OK: u32 = 200;
const STATUS_FORBIDDEN: u32 = 403;
const STATUS_NOT_FOUND: u32 = 404;
const STATUS_METHOD_NOT_ALLOWED: u32 = 405;


pub fn create_response_for_request(document_root: &str, request_raw: &str) -> Option<Response> {
    let split = request_raw.split(" ");

    let mut response: Response = Response::new();
    let request_fields: Vec<&str> = split.collect();
    if request_fields.len() != 3 {
        return None;
    }

    response.headers.push(format!("Date: {}", Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string()));
    response.headers.push(format!("Server: {}", "rust static server"));
    response.headers.push("Connection: close".to_string());

    response.file = None;
    response.status = Some(STATUS_NOT_FOUND);

    let url =  percent_decode(request_fields[1].as_bytes()).decode_utf8().unwrap();
    let url = url.split("?").next().unwrap();
    let path_string = format!("{}{}", document_root, url);
    println!("path_string - {}", path_string);
    let path = Path::new(&path_string);

    let fullpath;
    if path.is_dir() {
        response.status = Some(STATUS_FORBIDDEN);
        fullpath = path.join("index.html");
    } else {
        fullpath = path.to_path_buf()
    }

    let path = match fullpath.canonicalize() {
        Ok(p) => Some(p),
        Err(_) => None
    };

    if path == None {
        return Some(response);
    }

    let path = path.unwrap();

    if !path.starts_with(document_root) {
        return Some(response);
    }

    match request_fields[0] {
        "GET" => {
            match File::open(path.clone()) {
                Ok(f) => {
                    response.headers.push(format!("Content-Type: {}", get_content_type(path.extension().unwrap())));
                    response.headers.push(format!("Content-Length: {}", path.metadata().unwrap().len().to_string()));

                    response.file = Some(f);
                    response.status = Some(STATUS_OK);
                }
                Err(_) => {}
            }
        }
        "HEAD" => {
            match path.exists() {
                true => {
                    response.headers.push(format!("Content-Type: {}", get_content_type(path.extension().unwrap())));
                    response.headers.push(format!("Content-Length: {}", path.metadata().unwrap().len().to_string()));

                    response.status = Some(STATUS_OK);
                }
                false => {}
            }
        }
        _ => {
            response.status = Some(STATUS_METHOD_NOT_ALLOWED);
        }
    }

    Some(response)
}

fn get_content_type(ext: &OsStr) -> &str {
    match ext.to_str().unwrap() {
        "html" => {
            "text/html"
        }
        "css" => {
            "text/css"
        }
        "js" => {
            "application/javascript"
        }
        "jpg" | "jpeg" => {
            "image/jpeg"
        }
        "png" => {
            "image/png"
        }
        "gif" => {
            "image/gif"
        }
        "swf" => {
            "application/x-shockwave-flash"
        }
        _ => {
            "application/octet-stream"
        }
    }
}