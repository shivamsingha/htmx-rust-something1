use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub logo_url: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct CompanyIdName {
    pub id: i32,
    pub name: String,
}
