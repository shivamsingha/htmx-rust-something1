use axum::{
    extract::{Path, Query},
    Extension,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::{
    models::job::Job,
    templates::{JobTemplate, ListJobsTemplate},
};

pub async fn get_job(
    Path(id): Path<i32>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> JobTemplate {
    let job = sqlx::query_as!(Job, "SELECT * FROM jobs WHERE id = $1", id)
        .fetch_one(&conn)
        .await
        .unwrap();

    JobTemplate { job }
}

#[derive(Deserialize)]
pub struct Pagination {
    page: i64,
    per_page: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}

pub async fn list_jobs(
    pagination: Option<Query<Pagination>>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> ListJobsTemplate {
    let Query(pagination) = pagination.unwrap_or_default();

    let jobs = sqlx::query_as!(
        Job,
        "SELECT * FROM jobs WHERE expires_at > NOW() OR expires_at IS NULL ORDER BY id LIMIT $1 OFFSET $2",
        pagination.per_page,
        (pagination.page - 1) * pagination.per_page
    )
    .fetch_all(&conn)
    .await
    .unwrap();

    ListJobsTemplate { jobs }
}

pub async fn update_jobs(
    Path(id): Path<i32>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> JobTemplate {
    let job = sqlx::query_as!(
        Job,
        "UPDATE jobs SET expires_at = NOW() WHERE id = $1 RETURNING *",
        id
    )
    .fetch_one(&conn)
    .await
    .unwrap();

    JobTemplate { job }
}

pub async fn create_job(Extension(conn): Extension<Pool<Postgres>>) -> JobTemplate {
    let job = sqlx::query_as!(
        Job,
        "INSERT INTO jobs (title, description, company_id) VALUES ('New Job', 'New Job Description', 1) RETURNING *"
    )
    .fetch_one(&conn)
    .await
    .unwrap();

    JobTemplate { job }
}

pub async fn delete_job(
    Path(id): Path<i32>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> JobTemplate {
    let job = sqlx::query_as!(Job, "DELETE FROM jobs WHERE id = $1 RETURNING *", id)
        .fetch_one(&conn)
        .await
        .unwrap();

    JobTemplate { job }
}
