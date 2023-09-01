use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn hello() -> Result<impl IntoResponse, std::convert::Infallible> {
    let hello = HelloTemplate { name: "world" };
    Ok(hello)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .nest_service("/static", ServeDir::new("static"));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
