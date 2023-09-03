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
