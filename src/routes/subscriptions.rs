use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
