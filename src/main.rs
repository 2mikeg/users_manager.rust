use std::time::Duration;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod conf;
mod db;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let settings = conf::load_env();

    let db_conn_url = db::get_db_conn_str(settings);

    let _ = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(15))
        .connect(&db_conn_url)
        .await
        .expect("can't connect to db");

    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
