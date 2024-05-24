use tokio;
use crate::auth_service::{
    auth_client::AuthClient, 
    AuthenticationAnswerRequest,
    RegisterRequest,
    AuthenticationAnswerResponse
};

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_auth_server() {
        let mut client = AuthClient::connect("http://127.0.0.1:50051")
            .await
            .expect("could not connect to the server");
        println!("✅ Connected to the server");

        let username = "Guido";
        let password = "Guido";

        let register_request = RegisterRequest {
            user: String::from(username),
            password: String::from(password)
        };

        let register_response = client
            .register(register_request)
            .await
            .expect("Could not register in server");

        let register_response = &register_response.into_inner();
        let y2 = register_response.y2.clone();
        let y1 = register_response.y1.clone();

        let auth_request = AuthenticationAnswerRequest {
            user: String::from(username),
            password: String::from(password),
            y1: y1,
            y2: y2
        };

        let auth_response = client
            .verify_authentication(auth_request)
            .await
            .expect("Could not verify authentication in server")
            .into_inner();

        assert_eq!(AuthenticationAnswerResponse {}, auth_response)


    }
}