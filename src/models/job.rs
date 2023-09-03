use sqlx::{FromRow, types::{chrono, BigDecimal}};

#[derive(Debug, FromRow)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub company_id: i32,
    pub location: Option<String>,
    pub salary: Option<BigDecimal>,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: Option<chrono::NaiveDateTime>,
}