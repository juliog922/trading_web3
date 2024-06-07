use actix_web::web::Data;
use actix_web::{test, App};
use serde_json::json;

use crate::routes::register::register;
use crate::initialize_db_pool;

#[actix_web::test]
async fn test_register_post_assert() {

    let payload = json!({
        "user": "Test_user1",
        "password": "Test_password123456"
    });
    let pool = initialize_db_pool();
    let mut app = test::init_service(App::new().app_data(Data::new(pool.clone())).service(register)).await;

    let resp = test::TestRequest::post()
        .append_header(
            actix_web::http::header::ContentType::json()
        )
        .uri("/register")
        .set_json(&payload)
        .send_request(&mut app)
        .await;

    // Verificar la respuesta original
    assert!(resp.status().is_success(), "Failed to register user: {:?}", resp);
}

#[actix_web::test]
async fn test_register_post_fail() {

    let payload = json!({
        "user": "Test_user",
        "password": "Test_password"
    });
    let pool = initialize_db_pool();
    let mut app = test::init_service(App::new().app_data(Data::new(pool.clone())).service(register)).await;

    let resp = test::TestRequest::post()
        .append_header(
            actix_web::http::header::ContentType::json()
        )
        .uri("/register")
        .set_json(&payload)
        .send_request(&mut app)
        .await;


    // Verificar la respuesta original
    assert!(resp.status().is_client_error(), "Success to register user: {:?}", resp);
}