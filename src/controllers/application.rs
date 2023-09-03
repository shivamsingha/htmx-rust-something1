use axum::{extract::Path, Extension};
use sqlx::{Pool, Postgres};

use crate::{models::application::Application, templates::ApplicationTemplate};

pub async fn get_application(
    Path(id): Path<i32>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> ApplicationTemplate {
    let application = sqlx::query_as!(Application, "SELECT * FROM applications WHERE id = $1", id)
        .fetch_one(&conn)
        .await
        .unwrap();

    ApplicationTemplate { application }
}
