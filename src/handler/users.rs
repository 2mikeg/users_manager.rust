use actix_web::{post, web, HttpResponse, Responder};
use sqlx;

use crate::conf;
use crate::model;

use password_auth::generate_hash;

#[post("/users")]
async fn create_user(
    app_state: web::Data<conf::AppState>,
    data: web::Json<model::users::CreateUser>,
) -> impl Responder {
    let hashed_password = generate_hash(data.password.as_str());

    let create_user_query = sqlx::query_as!(
        model::users::User,
        r#"INSERT INTO users (username, hashed_password) VALUES ($1, $2) RETURNING id, username, hashed_password, created_at;
        "#,
        data.username,
        hashed_password
    )
    .fetch_one(&app_state.db_pool)
    .await;

    if create_user_query.is_err() {
        let err = create_user_query.err().unwrap();

        let response = model::http::HTTPError {
            error: err.to_string(),
        };
        return HttpResponse::InternalServerError().json(response);
    }

    let response = create_user_query.unwrap();

    return HttpResponse::Created().json(response);
}
