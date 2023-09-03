use axum::{extract::Path, Extension};
use sqlx::{Pool, Postgres};

use crate::{models::applicant::Applicant, templates::ApplicantTemplate};

pub async fn get_applicant(
    Path(id): Path<i32>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> ApplicantTemplate {
    let applicant = sqlx::query_as!(Applicant, "SELECT * FROM applicants WHERE id = $1", id)
        .fetch_one(&conn)
        .await
        .unwrap();

    ApplicantTemplate { applicant }
}
