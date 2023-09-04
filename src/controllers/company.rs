use axum::{extract::Path, Extension, Form};
use sqlx::{Pool, Postgres};

use crate::{models::company::Company, templates::CompanyTemplate};

pub async fn get_company(
    Path(id): Path<i32>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> CompanyTemplate {
    let company = sqlx::query_as!(Company, "SELECT * FROM companies WHERE id = $1", id)
        .fetch_one(&conn)
        .await
        .unwrap();

    CompanyTemplate { company }
}

pub async fn create_company(
    Form(form_data): Form<Company>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> CompanyTemplate {
    let company = sqlx::query_as!(
            Company,
            "INSERT INTO companies (name, description, website, logo_url) VALUES ($1, $2, $3, $4) RETURNING *",
            form_data.name,
            form_data.description,
            form_data.website,
            form_data.logo_url
        ).fetch_one(&conn).await.unwrap();

    CompanyTemplate { company }
}

pub async fn update_company(
    Path(id): Path<i32>,
    Form(form_data): Form<Company>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> CompanyTemplate {
    let company = sqlx::query_as!(
            Company,
            "UPDATE companies SET name = $1, description = $2, website = $3, logo_url = $4 WHERE id = $5 RETURNING *",
            form_data.name,
            form_data.description,
            form_data.website,
            form_data.logo_url,
            id
        ).fetch_one(&conn).await.unwrap();

    CompanyTemplate { company }
}

pub async fn delete_company(
    Path(id): Path<i32>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> CompanyTemplate {
    let company = sqlx::query_as!(
        Company,
        "DELETE FROM companies WHERE id = $1 RETURNING *",
        id
    )
    .fetch_one(&conn)
    .await
    .unwrap();

    CompanyTemplate { company }
}
