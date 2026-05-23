use axum::response::Html;
use std::fs;

pub async fn dashboard() -> Html<String> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/static/dashboard/dashboard.html"
    );
    let html = fs::read_to_string(path).expect("static file should be read");

    Html(html)
}
