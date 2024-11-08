use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    // Let's generate a random unique identifier
    let request_id = Uuid::new_v4();

    let request_sapn = tracing::info_span!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        %request_id,
        subscribe_email = %form.email,
        subscribe_name = %form.name
    );
    let _request_span_guard = request_sapn.enter();

    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );

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
    .execute(pool.get_ref())
    .instrument(query_span)
    .await 
    {
        Ok(_) => { 
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
           HttpResponse::InternalServerError().finish()
        }
    }
}