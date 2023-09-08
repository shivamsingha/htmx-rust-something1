use axum::{
    extract::{Path, Query},
    Extension,
};
use serde::{Deserialize, Deserializer};
use sqlx::{types::BigDecimal, Pool, Postgres};

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
    #[serde(default, deserialize_with = "deserialize_option_string")]
    search: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string")]
    location: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i32")]
    company_id: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_option_string")]
    sort_by: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string")]
    sort_direction: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_big_decimal")]
    min_salary: Option<BigDecimal>,
    #[serde(default, deserialize_with = "deserialize_option_big_decimal")]
    max_salary: Option<BigDecimal>,
    #[serde(default, deserialize_with = "deserialize_option_string")]
    is_active: Option<String>,
    #[serde(default)]
    page: Option<i64>,
    #[serde(default)]
    per_page: Option<i64>,
}

fn deserialize_option_big_decimal<'de, D>(deserializer: D) -> Result<Option<BigDecimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse().unwrap()))
    }
}

fn deserialize_option_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse().unwrap()))
    }
}

fn deserialize_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

pub async fn list_jobs(
    Query(params): Query<Params>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> ListJobsTemplate {
    let jobs = sqlx::query_as!(
        JobWithCompany,
        "SELECT jobs.*,
            companies.name AS company_name
            FROM jobs
            INNER JOIN companies ON jobs.company_id = companies.id
            WHERE (
                jobs.title ILIKE '%' || COALESCE($1, '%') || '%'
                OR jobs.description ILIKE '%' || COALESCE($1, '%') || '%'
                OR jobs.location ILIKE '%' || COALESCE($1, '%') || '%'
                OR companies.name ILIKE '%' || COALESCE($1, '%') || '%'
            )
            AND (jobs.location = COALESCE($2, jobs.location))
            AND (jobs.company_id = COALESCE($3, jobs.company_id))
            AND (jobs.salary >= COALESCE($7, jobs.salary))
            AND (jobs.salary <= COALESCE($8, jobs.salary))
            AND (
                jobs.expires_at > 
                CASE
                    WHEN $9 = 'on' THEN NOW()
                    ELSE '1980-01-01 00:00:00'
                END
                OR jobs.expires_at IS NULL
            )
            ORDER BY $4 OFFSET $5 ROWS FETCH NEXT $6 ROWS ONLY",
        params.search,
        params.location,
        params.company_id,
        params.sort_by,
        (params.page.map(|p| (p - 1) * params.per_page.unwrap_or(0))),
        params.per_page,
        params.min_salary,
        params.max_salary,
        params.is_active
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
