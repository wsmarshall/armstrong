use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    //generate a random unique identifier
    let request_id = Uuid::new_v4();
    //similar to logs, Spans have a "level"
    //'info_span' creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.", %request_id, subscriber_email = %form.email, subscriber_name = %form.name);
    let _request_span_guard = request_span.enter();

    //no need to call 'enter' on query_span
    //'.instrument' does that automatically in the future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    //use 'get_ref' to get immutable reference to the
    //PgConnection which is wrapped by web::Data=
    .execute(pool.get_ref())
    //attach instrumentation first, then await
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
