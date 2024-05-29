use num_bigint::BigUint;
use tonic::{Code, Request, Response, Status};
use bip39::{
    Mnemonic,
    Language
};
use crate::auth::zkp::ZKP;
use crate::auth::auth_service::{
    auth_server::Auth,
    AuthenticationAnswerRequest, 
    AuthenticationAnswerResponse,
    RegisterRequest, 
    RegisterResponse,
    PasswordRecoveryRequest,
    PasswordRecoveryResponse
};

/// Implementation of the authentication service.
#[derive(Debug, Default)]
pub struct AuthImpl {}

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

        let mut password_bytes = password.to_bytes_be();
        if password_bytes.len() < 32 {
            password_bytes.resize(32, 0);
        } else if password_bytes.len() > 32 {
            password_bytes.truncate(32);
        }
        
        let mnemonic = match Mnemonic::from_entropy(&password_bytes, Language::English) {
            Ok(mnemonic) => mnemonic,
            Err(_) => return Err(Status::new(Code::InvalidArgument, "Invalid password"))
        };
        let secret_phrase = mnemonic.phrase().to_string();

        let (alpha, beta, p, _) = ZKP::get_constants();
        let (y1, y2) = ZKP::compute_pair(&alpha, &beta, &p, &password);

        let y1 = y1.to_bytes_be();
        let y2 = y2.to_bytes_be();

        println!("✅ Successful Registration username: {:?}", user_name);
        Ok(Response::new(RegisterResponse { secret_phrase, y1, y2}))
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

    async fn password_recovery(
        &self,
        request: Request<PasswordRecoveryRequest>,
    ) -> Result<Response<PasswordRecoveryResponse>, Status> {
        let request = request.into_inner();
        let secret_phrase = request.secret_phrase;

        let mnemonic = match Mnemonic::from_phrase(&secret_phrase, Language::English) {
            Ok(mnemonic) => mnemonic,
            Err(_) => return Err(Status::new(Code::InvalidArgument, "Invalid mnemonic phrase")),
        };

        let entropy: &[u8] = mnemonic.entropy();
        match std::str::from_utf8(entropy) {
            Ok(password) => {
                println!("{}", password);
                if let Some(pos) = password.find('\0') {
                // Cortar la cadena hasta esa posición
                let password = &password[..pos];
                Ok(Response::new(PasswordRecoveryResponse{password: password.to_string()}))
                } else {
                    Ok(Response::new(PasswordRecoveryResponse{password: password.to_string()}))
                }
            },
            Err(_) => Err(Status::new(
                Code::PermissionDenied,
                "Error: Invalid UTF-8 sequence",
            )),
        }
        
    }
}