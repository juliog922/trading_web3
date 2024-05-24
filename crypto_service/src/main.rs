use tonic::transport::Server;
pub mod test;
pub mod auth_service {
    include!("./auth/auth_service.rs");
}
pub mod auth{
    pub mod authimpl;
    pub mod zkp;
}
use crate::auth::authimpl::AuthImpl;
use crate::auth_service::auth_server::AuthServer;

/// Main function to start the gRPC server.
///
/// This function initializes and starts the gRPC server. It configures the server
/// to listen on the specified address and port, initializes the authentication implementation,
/// creates a new instance of the authentication server, and then serves the server.
#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:50051".to_string();

    println!("✅ Running the server in {}", addr);

    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("could not convert address"))
        .await
        .unwrap();
}