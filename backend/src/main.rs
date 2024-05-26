use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::PgConnection;
use diesel::{r2d2};

mod routes;
mod schema;
mod models;
mod repositories;
mod auth;

use crate::routes::{
    register::register,
    authorize::authorize,
};

pub type DbConn = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

fn initialize_db_pool() -> DbConn {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(register)
            .service(authorize)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}