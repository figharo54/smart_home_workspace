use actix_web::{App, test, web};
use smart_home_backend::{config, model::AppState};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn state() -> AppState {
    Arc::new(Mutex::new(HashMap::new()))
}

#[actix_web::test]
async fn create_and_get_room() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state()))
            .configure(config),
    )
    .await;

    // Создать комнату
    let req = test::TestRequest::post()
        .uri("/rooms")
        .set_json(serde_json::json!({"name": "Кухня"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    let id = body["id"].as_str().unwrap();

    // Получить комнату
    let req = test::TestRequest::get()
        .uri(&format!("/rooms/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn get_nonexistent_room_returns_404() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state()))
            .configure(config),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/rooms/00000000-0000-0000-0000-000000000000")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
