use axum::{
    extract::{Multipart, Path},
    Extension, Form,
};

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
    mut multipart: Multipart,
    Extension(conn): Extension<Pool<Postgres>>,
) -> Result<CompanyTemplate, anyhow::Error> {
    let mut form_data = Company {
        id: 0,
        name: String::new(),
        description: None,
        website: None,
        logo_url: None,
    };

    while let Some(field) = multipart.next_field().await? {
        let name = field.name();
        match name {
            Some("name") => form_data.name = String::from_utf8(field.bytes().await?.to_vec())?,
            Some("description") => {
                form_data.description = Some(String::from_utf8(field.bytes().await?.to_vec())?)
            }
            Some("website") => {
                let field_data = field.bytes().await?;
                let website = std::str::from_utf8(&field_data)?;
                form_data.website = Some(url::Url::parse(website)?.as_str().to_string());
            }
            _ => (), // Ignore other fields
        }
    }

    let company = sqlx::query_as!(
        Company,
        "INSERT INTO companies (name, description, website, logo_url) VALUES ($1, $2, $3, $4) RETURNING *",
        form_data.name,
        form_data.description,
        form_data.website,
        form_data.logo_url
    ).fetch_one(&conn).await.unwrap();

    Ok(CompanyTemplate { company })
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
