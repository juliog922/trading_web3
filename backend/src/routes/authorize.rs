use actix_jwt_auth_middleware::TokenSigner;
use jwt_compact::alg::Ed25519;
use actix_web::{post, web, HttpResponse, Responder, Result, error};
use serde::Deserialize;
use crate::auth::auth_service::{
    auth_client::AuthClient, 
    AuthenticationAnswerRequest
};
use crate::{DbConn, JWTUser, AttempsData};
use crate::repositories::users::UsersRepository;

#[derive(Deserialize, Clone)]
pub struct AuthorizeRequestSchema {
    user: String,
    password: String,
}

#[post("/authorize")]
pub async fn authorize(
    pool: web::Data<DbConn>, 
    req: web::Json<AuthorizeRequestSchema>,
    attemps_data: web::Data<AttempsData>,
    cookie_signer: web::Data<TokenSigner<JWTUser, Ed25519>>) -> Result<impl Responder> {

    let username = req.user.clone();
    let password = req.password.clone();

    let user = web::block({
        let username = username.clone();
        let pool = pool.clone();
        move || {
            let mut conn = pool.get()?;
            UsersRepository::find_user_by_username(&mut conn, &username)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if let Some(found_user) = user.clone() {
        if found_user.state == "Inactive".to_string() {
            return Ok(HttpResponse::Forbidden().body("You have to recover your password."));
        }
    }

    match user {
        Some(user) => {
            let y1 = user.y1.clone();
            let y2 = user.y2.clone();

            // Connect to the gRPC server
            let mut client = AuthClient::connect("http://crypto_service:50051")
                .await
                .expect("could not connect to crypto service");

            // Create an AuthenticationAnswerRequest
            let auth_request = AuthenticationAnswerRequest {
                user: username.clone(),
                password: password.clone(),
                y1: y1,
                y2: y2,
            };

            // Send authentication request to the server
            match client
            .verify_authentication(auth_request)
            .await {
                Ok(_) => {
                    let jwt_user = JWTUser {username: username.clone()};

                    Ok(HttpResponse::Ok()
                        .cookie(cookie_signer.create_access_cookie(&jwt_user)?)
                        .cookie(cookie_signer.create_refresh_cookie(&jwt_user)?)
                        .body("You are now logged in"))
                },
                Err(_) => {
                    let mut attemps_data = attemps_data.lock().await;
                    let counter = attemps_data.entry(username.clone()).or_insert(0);
                    *counter += 1;

                    if *counter >= 3 {
                        attemps_data.remove(&username);

                        web::block({
                            let username = username.clone();

                            move || {
                                let mut conn = pool.get()?;
                                UsersRepository::change_user_state(&mut conn, &username, "Inactive")
                            }})
                            .await?
                            .map_err(error::ErrorInternalServerError)?;

                        Ok(HttpResponse::Forbidden().body("You have reached the maximum number of failed attempts."))
                    } else {
                        Ok(HttpResponse::Ok().body(format!("Failed attempts to {}: {}", username, *counter)))
                    }
                }
            } 
        }
        None => {
            
            Ok(HttpResponse::NotFound().body(format!("No user found with Username: {username}")))
        },
    }
}