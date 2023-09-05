use axum::{routing::get, Extension, Router};
use dotenvy_macro::dotenv;
use htmx_rust_something1::controllers::{
    applicant::get_applicant,
    application::get_application,
    company::get_company,
    home::home,
    job::{get_job, list_jobs},
};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    let database_url = dotenv!("DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(home))
        .route("/company/:id", get(get_company))
        .route("/job/:id", get(get_job))
        .route("/jobs", get(list_jobs))
        .route("/applicant/:id", get(get_applicant))
        .route("/application/:id", get(get_application))
        .nest_service("/static", ServeDir::new("static"))
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
        .layer(Extension(pool));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
