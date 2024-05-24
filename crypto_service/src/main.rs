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