use actix_web::{post, web, HttpResponse, Responder, Result, error};
use serde::{Deserialize, Serialize};
use crate::auth::auth_service::{
    auth_client::AuthClient, 
    RegisterRequest,    
};
use crate::DbConn;
use crate::models::users::NewUser;
use crate::repositories::users::UsersRepository;
use crate::utils::validation::validate_password;


#[derive(Deserialize, Clone, Serialize)]
pub struct RegisterRequestSchema {
    pub user: String,
    pub password: String
}



#[post("/register")]
pub async fn register(pool: web::Data<DbConn>, req: web::Json<RegisterRequestSchema>) -> Result<impl Responder> {
    let user = req.user.clone();
    let password = req.password.clone();

    if !validate_password(&password) {
        return Ok(HttpResponse::Unauthorized().json("Invalid password"));
    }

    let username = user.clone();

    let first_pool = pool.clone();
    let existed_user = web::block({
        let username = username.clone();
        move || {
            let mut conn = first_pool.get()?;
            UsersRepository::find_user_by_username(&mut conn, &username)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match existed_user {
        Some(_) => {return Ok(HttpResponse::Created().json("User created Succesfully"));},
        None => {},
    };

    // Connect to the gRPC server
    let mut client = AuthClient::connect("http://crypto_service:50051")
        .await
        .expect("could not connect to crypto service");

    // Create a RegisterRequest
    let register_request = RegisterRequest {
        user: user.clone(),
        password: password.clone()
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
        username:user,
        y1:y1,
        y2:y2
    };

    // use web::block to offload blocking Diesel queries without blocking server thread
    let _ = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        UsersRepository::create(&mut conn, new_user)
    })
        .await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(secret_phrase))
}