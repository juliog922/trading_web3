use actix_web::{put, web, HttpResponse, Responder, Result, error};
use serde::Deserialize;
use crate::auth::auth_service::{
    auth_client::AuthClient, 
    PasswordRecoveryRequest,
    RegisterRequest    
};
use crate::DbConn;
use crate::models::users::NewUser;
use crate::repositories::users::UsersRepository;
use crate::utils::validation::validate_password;

#[derive(Deserialize, Clone)]
pub struct ResetDataSchema {
    user: String,
    secret_phrase: String,
    new_password: String
}

#[put("/reset")]
pub async fn reset(pool: web::Data<DbConn>, req: web::Json<ResetDataSchema>) -> Result<impl Responder> {
    let username = req.user.clone();
    let secret_phrase = req.secret_phrase.clone();
    let new_password = req.new_password.clone();

    if !validate_password(&new_password) {
        return Ok(HttpResponse::Unauthorized().json("Invalid password"));
    }

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

                    let mut client = AuthClient::connect("http://crypto_service:50051")
                        .await
                        .expect("could not connect to crypto service");

                    let register_request = RegisterRequest {
                        user: username.clone(),
                        password: new_password.clone()
                    };
                    // Send register request to the server
                    let register_response = client
                    .register(register_request)
                    .await
                    .expect("Could not register in server");

                    // Extract y2 and y1 from register response
                    let register_response = &register_response.into_inner();
                    let secret_phrase = register_response.secret_phrase.clone();
                    let y2 = register_response.y2.clone();
                    let y1 = register_response.y1.clone();

                    let new_user = NewUser {
                        username:username.clone(),
                        y1:y1,
                        y2:y2
                    };
                    // use web::block to offload blocking Diesel queries without blocking server thread
                    let _ = web::block(move || {
                        // note that obtaining a connection from the pool is also potentially blocking
                        let mut conn = pool.get()?;

                        UsersRepository::update_user(&mut conn, new_user)
                    })
                        .await?
                        // map diesel query errors to a 500 error response
                        .map_err(error::ErrorInternalServerError)?;

                    // user was added successfully; return 201 response with new user info
                    Ok(HttpResponse::Created().json(secret_phrase))
                },
                Err(_) => {return Ok(HttpResponse::Unauthorized().json("Invalid secret phrase"));}
            }   
        },
        None => Ok(HttpResponse::NotFound().body(format!("No user found with Username: {username}")))
    }
}