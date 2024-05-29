use actix_web::{post, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use crate::auth::auth_service::{
    auth_client::AuthClient, 
    PasswordRecoveryRequest,
    RegisterRequest    
};
use crate::DbConn;
use crate::repositories::users::UsersRepository;
use actix_web::error;

#[derive(Deserialize, Clone)]
struct PasswordRecoverySchema {
    user: String,
    secret_phrase: String
}

#[post("/recover")]
pub async fn recover(
    pool: web::Data<DbConn>, 
    req: web::Json<PasswordRecoverySchema>
) -> Result<impl Responder> {
    let username = req.user.clone();
    let secret_phrase = req.secret_phrase.clone();

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
        if found_user.state == "Active".to_string() {
            return Ok(HttpResponse::Forbidden().body("You have an active password."));
        }
    }

    match user {
        Some(_) => {
            let mut client = AuthClient::connect("http://crypto_service:50051")
                .await
                .expect("could not connect to crypto service");

            let recover_request = PasswordRecoveryRequest {
                secret_phrase: secret_phrase.clone()
            };

            match client.password_recovery(recover_request).await {
                Ok(recovery_response) => {
                    let password = recovery_response.into_inner().password; 

                    let mut client = AuthClient::connect("http://crypto_service:50051")
                        .await
                        .expect("could not connect to crypto service");

                    let register_request = RegisterRequest {
                        user: username.clone(),
                        password: password.clone()
                    };
                    // Send register request to the server
                    let register_response = client
                    .register(register_request)
                    .await
                    .expect("Could not register in server");

                    // Extract y2 and y1 from register response
                    let register_response = &register_response.into_inner();
                    let y2 = register_response.y2.clone();
                    let y1 = register_response.y1.clone();

                    let (y1_store, y2_store) = web::block({
                        let username = username.clone();
                        let pool_cloned = pool.clone();

                        move || {
                            let mut conn = pool_cloned.get()?;
                            UsersRepository::get_y1_y2(&mut conn, &username)
                        }
                    })
                        .await?
                        .map_err(error::ErrorInternalServerError)?;

                    if (y1, y2) != (y1_store, y2_store) {
                        return Ok(HttpResponse::Unauthorized().json("Invalid secret phrase"));
                    }

                    web::block({
                        let username = username.clone();

                        move || {
                            let mut conn = pool.get()?;
                            UsersRepository::change_user_state(&mut conn, &username, "Active")
                        }
                    })
                        .await?
                        .map_err(error::ErrorInternalServerError)?;

                    return Ok(HttpResponse::Created().json(password));
                },
                Err(_) => {return Ok(HttpResponse::Unauthorized().json("Invalid secret phrase"));}
            }   
        },
        None => Ok(HttpResponse::NotFound().body(format!("No user found with Username: {username}")))
    }
}