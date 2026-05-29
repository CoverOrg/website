use axum::response::Html;
use tera::{Context, Tera};

pub async fn home() -> Html<String> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("tera new instance creation was expected");

    let context = Context::new();
    let rendered = tera
        .render("index.html", &context)
        .expect("it should have rendered successfully");

    Html(rendered)
}
