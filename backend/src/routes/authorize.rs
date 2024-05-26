use actix_web::{post, web, HttpResponse, Responder, Result, error};
use serde::Deserialize;
use crate::auth::auth_service::{
    auth_client::AuthClient, 
    AuthenticationAnswerRequest,
};
use crate::DbConn;
use crate::repositories::users::UsersRepository;

#[derive(Deserialize, Clone)]
struct AuthorizeRequestSchema {
    user: String,
    password: String,
}

#[post("/authorize")]
pub async fn authorize(pool: web::Data<DbConn>, req: web::Json<AuthorizeRequestSchema>) -> Result<impl Responder> {
    let username = req.user.clone();
    let password = req.password.clone();

    let user = web::block({
        let username = username.clone();
        move || {
            let mut conn = pool.get()?;
            UsersRepository::find_user_by_username(&mut conn, &username)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

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
            let _ = client
                .verify_authentication(auth_request)
                .await
                .expect("Could not verify authentication in server")
                .into_inner();

            Ok(HttpResponse::Ok().json(user))
        }
        None => Ok(HttpResponse::NotFound().body(format!("No user found with Username: {username}"))),
    }
}