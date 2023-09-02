use axum::{extract::Path, Extension};

use crate::{models::company::Company, templates::CompanyTemplate};

pub async fn get_company(
    Path(id): Path<i32>,
    Extension(conn): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> CompanyTemplate<'static> {
    let stream = sqlx::query_as!(Company, "SELECT * FROM companies WHERE id = $1", id)
        .fetch_one(&conn)
        .await
        .unwrap();
    CompanyTemplate { company: &stream }
}
