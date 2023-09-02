use sqlx::{FromRow, types::chrono};

#[derive(Debug, FromRow)]
pub struct Applicant {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub resume_url: Option<String>,
    pub cover_letter: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}