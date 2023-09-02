use sqlx::{FromRow, types::chrono};

#[derive(Debug, FromRow)]
pub struct Application {
    pub id: i32,
    pub job_id: i32,
    pub applicant_id: i32,
    pub created_at: chrono::NaiveDateTime,
}