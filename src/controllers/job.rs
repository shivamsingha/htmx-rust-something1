use axum::{
    extract::{Path, Query},
    Extension,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::{
    models::job::{Job, JobWithCompany},
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
pub struct Params {
    search: Option<String>,
    location: Option<String>,
    company_id: Option<i32>,
    sort_by: Option<String>,
    sort_direction: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}

pub async fn list_jobs(
    Query(params): Query<Params>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> ListJobsTemplate {
    let jobs = sqlx::query_as!(
        JobWithCompany,
        "SELECT jobs.*, companies.name AS company_name 
        FROM jobs 
        INNER JOIN companies ON jobs.company_id = companies.id 
        WHERE (jobs.title LIKE '%' || $1 || '%') 
        AND (jobs.location = COALESCE($2, jobs.location)) 
        AND (jobs.company_id = COALESCE($3, jobs.company_id))
        ORDER BY $4
        OFFSET $5 ROWS
        FETCH NEXT $6 ROWS ONLY",
        params.search,
        params.location,
        params.company_id,
        params.sort_by,
        params.page,
        params.per_page
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
