use http::{
    request::{HttpRequest, Rescourse, Version},
    response::{HttpResponse, HttpStatus},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};

pub trait Handler {
    fn handle(request: &HttpRequest) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let file_content = fs::read_to_string(full_path);

        file_content.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: u32,
    order_date: String,
    order_status: bool,
}

impl Handler for PageNotFoundHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        HttpResponse::new(
            Version::V1_1,
            HttpStatus::NOT_FOUND,
            None,
            Self::load_file("404.html"),
        )
    }
}

impl Handler for StaticPageHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let Rescourse::Path(s) = &request.resource;
        let paths: Vec<&str> = s.split("/").collect();

        match paths[1] {
            "" => HttpResponse::new(
                Version::V1_1,
                HttpStatus::OK,
                None,
                Self::load_file("index.html"),
            ),
            "health" => HttpResponse::new(
                Version::V1_1,
                HttpStatus::OK,
                None,
                Self::load_file("health.html"),
            ),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut headers: HashMap<String, String> = HashMap::new();

                    headers.insert("Content-Type".into(), "text/html".into());

                    if path.ends_with(".css") {
                        headers.insert("Content-Type".into(), "text/css".into());
                    } else if path.ends_with(".js") {
                        headers.insert("Content-Type".into(), "text/javascript".into());
                    }

                    HttpResponse::new(Version::V1_1, HttpStatus::OK, Some(headers), Some(contents))
                }
                None => HttpResponse::new(
                    Version::V1_1,
                    HttpStatus::NOT_FOUND,
                    None,
                    Self::load_file("404.html"),
                ),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, "orders.json");

        let json_content = fs::read_to_string(full_path);

        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_content.unwrap().as_str()).unwrap();

        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let Rescourse::Path(s) = &request.resource;
        let paths: Vec<&str> = s.split("/").collect();

        match paths[2] {
            "shipping" if paths.len() > 2 && paths[3] == "orders" => {
                let mut headers: HashMap<String, String> = HashMap::new();
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());

                headers.insert("Content-Type".into(), "application/json".into());

                HttpResponse::new(Version::V1_1, HttpStatus::OK, Some(headers), body)
            }
            _ => HttpResponse::new(
                Version::V1_1,
                HttpStatus::NOT_FOUND,
                None,
                Self::load_file("404.html"),
            ),
        }
    }
}
