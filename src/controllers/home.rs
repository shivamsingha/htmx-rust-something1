use axum::Extension;
use sqlx::{Pool, Postgres};

use crate::{
    models::{company::CompanyIdName, job::JobLocation},
    templates::HomeTemplate,
};

pub async fn home(Extension(conn): Extension<Pool<Postgres>>) -> HomeTemplate {
    let companies = sqlx::query_as!(CompanyIdName, "SELECT id, name FROM companies")
        .fetch_all(&conn)
        .await
        .unwrap();

    let locations = sqlx::query_as!(
        JobLocation,
        "SELECT DISTINCT location as \"location!\" FROM jobs WHERE location IS NOT NULL"
    )
    .fetch_all(&conn)
    .await
    .unwrap();

    HomeTemplate {
        companies,
        locations,
    }
}
