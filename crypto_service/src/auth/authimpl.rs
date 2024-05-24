use std::{collections::HashMap, sync::Mutex};
use num_bigint::BigUint;
use tonic::{Code, Request, Response, Status};
use crate::auth::zkp::ZKP;
use crate::auth_service::{
    auth_server::Auth,
    AuthenticationAnswerRequest, AuthenticationAnswerResponse,RegisterRequest, RegisterResponse,
};
#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_user: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct UserInfo {
    // registration
    pub user_name: String,
    pub y1: BigUint,
    pub y2: BigUint,
    // authorization
    pub r1: BigUint,
    pub r2: BigUint,
    // verification
    pub c: BigUint,
    pub s: BigUint,
    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();

        let user_name = request.user;
        println!("Processing Registration username: {:?}", &user_name);

        let password = BigUint::from_bytes_be(request.password.trim().as_bytes());
        
        let (alpha, beta, p, _) = ZKP::get_constants();
        let (y1, y2) = ZKP::compute_pair(&alpha, &beta, &p, &password);

        let y1 = y1.to_bytes_be();
        let y2 = y2.to_bytes_be();

        println!("✅ Successful Registration username: {:?}", user_name);
        Ok(Response::new(RegisterResponse { y1, y2}))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let request = request.into_inner();

        let user_name = request.user;
        let y1 = BigUint::from_bytes_be(&request.y1);
        let y2 = BigUint::from_bytes_be(&request.y2);
        let password = BigUint::from_bytes_be(request.password.trim().as_bytes());
        println!("Processing Challenge Solution auth_id: {:?}", user_name);

        let (alpha, beta, p, q) = ZKP::get_constants();
        let k = ZKP::generate_random_number_below(&q);
        let (r1, r2) = ZKP::compute_pair(&alpha, &beta, &p, &k);
        let c = ZKP::generate_random_number_below(&q);

        let zkp = ZKP { alpha, beta, p, q };
        let s = zkp.solve(&k, &c, &password);

        let verification = zkp.verify(
            &r1,
            &r2,
            &y1,
            &y2,
            &c,
            &s,
        );

        if verification {
            
            println!("✅ Correct Challenge Solution username: {:?}", user_name);

            Ok(Response::new(AuthenticationAnswerResponse {}))
        } else {
            println!("❌ Wrong Challenge Solution username: {:?}", user_name);

            Err(Status::new(
                Code::PermissionDenied,
                format!("User: {} bad solution to the challenge", user_name),
            ))
        }
    }
}