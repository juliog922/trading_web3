use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, FromRequest, TokenSigner};
use serde::{Deserialize, Serialize};
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::PgConnection;
use diesel::r2d2;

mod routes;
mod schema;
mod models;
mod repositories;
mod auth;
mod utils;

use crate::routes::{
    register::register,
    authorize::authorize,
    recover::recover,
    reset::reset
};

pub type DbConn = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;
type AttempsData = Arc<Mutex<HashMap<String, u32>>>;

fn initialize_db_pool() -> DbConn {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRequest)]
struct JWTUser {
    username: String,
}

#[get("/hello")]
async fn hello(jwt_user: JWTUser) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello there, i see your username is {}.", &jwt_user.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let KeyPair {
        pk: public_key,
        sk: secret_key,
    } = KeyPair::generate();

    let pool = initialize_db_pool();
    let attemps_data: AttempsData = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        let authority = Authority::<JWTUser, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(secret_key.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect(""),
            ))
            .verifying_key(public_key)
            .build()
            .expect("");

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(attemps_data.clone()))
            // Add routes that do not require JWT authentication
            .service(register)
            .service(authorize)
            .service(recover)
            .service(reset)
            // Add the protected scope
            .use_jwt(authority.clone(), web::scope("/protected").service(hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}