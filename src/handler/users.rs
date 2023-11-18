use actix_web::{delete, get};
use actix_web::{post, web, HttpResponse, Responder};
use sqlx;
use uuid::Uuid;

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

#[get("/users/{user_id}")]
async fn get_users(
    app_state: web::Data<conf::AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id_str = path.into_inner();
    let user_id_uuid = Uuid::parse_str(&user_id_str);

    let get_user_query = sqlx::query_as!(
        model::users::User,
        r#"SELECT * FROM users WHERE id=($1);"#,
        user_id_uuid.unwrap()
    )
    .fetch_one(&app_state.db_pool)
    .await;

    if get_user_query.is_err() {
        let err = get_user_query.err().unwrap();

        let response = model::http::HTTPError {
            error: err.to_string(),
        };
        return HttpResponse::InternalServerError().json(response);
    }

    let response = get_user_query.unwrap();

    return HttpResponse::Ok().json(response);
}

#[delete("/users/{user_id}")]
async fn delete_user(
    app_state: web::Data<conf::AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id_str = path.into_inner();
    let user_id_uuid = Uuid::parse_str(&user_id_str);

    let delete_user_query =
        sqlx::query!(r#"DELETE FROM users WHERE id=($1);"#, user_id_uuid.unwrap())
            .execute(&app_state.db_pool)
            .await;

    if delete_user_query.is_err() {
        let err = delete_user_query.err().unwrap();

        let response = model::http::HTTPError {
            error: err.to_string(),
        };

        return HttpResponse::InternalServerError().json(response);
    }

    let _ = delete_user_query.unwrap();

    return HttpResponse::Ok().json(user_id_str);
}
