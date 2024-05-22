use std::{collections::HashMap, sync::Mutex};
use rand::Rng;
use num_bigint::{BigUint, RandBigInt};
use tonic::{transport::Server, Code, Request, Response, Status};

pub mod auth_service {
    include!("./auth_service.rs");
}

use auth_service::{
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse,RegisterRequest, RegisterResponse,
};

pub struct ZKP {
    pub p: BigUint,
    pub q: BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
}

impl ZKP {
    /// output = (alpha^exp mod p, beta^exp mod p)
    pub fn compute_pair(alpha: &BigUint, beta: &BigUint, p: &BigUint, password: &BigUint) -> (BigUint, BigUint) {
        let p1 = alpha.modpow(password, &p);
        let p2 = beta.modpow(password, &p);
        (p1, p2)
    }

    /// output = s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
        }
        &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q)
    }

    /// cond1: r1 = alpha^s * y1^c
    /// cond2: r2 = beta^s * y2^c
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let cond1 = *r1
            == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        let cond2 = *r2
            == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        cond1 && cond2
    }

    pub fn generate_random_number_below(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(bound)
    }

    pub fn generate_random_string(size: usize) -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint) {
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(
            &hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),
        );

        // beta = alpha^i is also a generator
        let exp = BigUint::from_bytes_be(&hex::decode("266FEA1E5C41564B777E69").unwrap());
        let beta = alpha.modpow(&exp, &p);

        (alpha, beta, p, q)
    }
}

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

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:50051".to_string();

    println!("✅ Running the server in {}", addr);

    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("could not convert address"))
        .await
        .unwrap();
}