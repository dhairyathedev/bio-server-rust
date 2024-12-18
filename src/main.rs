use axum::Router;
use axum::routing::get;
use axum::response::Json;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{self, Read};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(fetch_bio));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fetch_bio() -> Json<Value> {
    let filename = "src/bio.json";

    let content = match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            return Json(json!({
                "error": "Failed to read the file."
            }));
           }
    };

    let bio_data: Value = match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(_) => {
            return Json(json!({
                "error": "unexpected file, unable to parse the JSON"
            }));
        }
    };

    Json(json!({
        "general": bio_data,
    }))
}
