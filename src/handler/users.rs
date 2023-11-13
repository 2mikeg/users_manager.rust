extern crate crypto;

use actix_web::{post, web, HttpResponse, Responder};
use sqlx;

use crate::conf;
use crate::model;

#[post("/user")]
async fn create_user(
    app_state: web::Data<conf::AppState>,
    data: web::Data<model::users::CreateUser>,
) -> impl Responder {
    let create_user_query = sqlx::query_as!(
        model::users::User,
        r#"INSERT INTO users (username, hashed_password) VALUES ($1, $2)"#,
        data.username,
        data.password
    )
    .fetch_one(&app_state.db_pool)
    .await;

    if create_user_query.is_err() {
        return HttpResponse::InternalServerError();
    }

    return HttpResponse::Created();
}
