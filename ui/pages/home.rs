use axum::response::Html;
use std::fs;

pub async fn home() -> Html<String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/static/index.html");
    let html = fs::read_to_string(path).expect("static file should be read");

    Html(html)
}
