use std::time::Duration;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod conf;
mod db;
mod handler;
mod model;
mod routes;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let settings = conf::load_env();

    let db_conn_url = db::get_db_conn_str(settings);

    let db_pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(15))
        .connect(&db_conn_url)
        .await
        .map_err(|err| {
            eprintln!("Error connecting to the database: {}", err);
            std::process::exit(1)
        })
        .expect("can't connect to db");

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Migration does not run!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conf::AppState {
                db_pool: db_pool.clone(),
            }))
            .service(handler::users::create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
