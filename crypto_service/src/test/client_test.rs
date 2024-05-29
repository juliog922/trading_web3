use tokio;
use crate::auth::auth_service::{
    auth_client::AuthClient, 
    AuthenticationAnswerRequest,
    RegisterRequest,
    AuthenticationAnswerResponse,
    PasswordRecoveryRequest
};

/// Module containing unit tests for the authentication server.
#[cfg(test)]
mod test {
    use super::*;
    
    /// Test function for the authentication server.
    #[tokio::test]
    async fn test_auth_server() {
        // Connect to the gRPC server
        let mut client = AuthClient::connect("http://localhost:50051")
            .await
            .expect("could not connect to the server");
        println!("✅ Connected to the server");

        // Define username and password
        let username = "Guido";
        let password = "Guido";

        // Create a RegisterRequest
        let register_request = RegisterRequest {
            user: String::from(username),
            password: String::from(password)
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

        // Create an AuthenticationAnswerRequest
        let auth_request = AuthenticationAnswerRequest {
            user: String::from(username),
            password: String::from(password),
            y1: y1,
            y2: y2
        };

        // Send authentication request to the server
        let auth_response = client
            .verify_authentication(auth_request)
            .await
            .expect("Could not verify authentication in server")
            .into_inner();

        // Assert that the authentication response is empty (for example)
        assert_eq!(AuthenticationAnswerResponse {}, auth_response);

        // Create a PasswordRecoveryRequest
        let recovery_request = PasswordRecoveryRequest {
            secret_phrase: secret_phrase
        };
        // Send authentication request to the server
        let recovery_response = client
            .password_recovery(recovery_request)
            .await
            .expect("Could not verify authentication in server");

        let recovery_response = &recovery_response.into_inner();
        let password = recovery_response.password.clone();

        assert_eq!("Guido".to_string(), password)

    }
}
