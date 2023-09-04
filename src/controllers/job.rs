use axum::{extract::Path, Extension};
use sqlx::{Pool, Postgres};

use crate::{models::job::Job, templates::JobTemplate};

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
