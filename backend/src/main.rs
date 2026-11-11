use actix_web::{App, HttpServer, web};
use smart_home_backend::{config, model::AppState};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state: AppState = Arc::new(Mutex::new(HashMap::new()));
    println!("Бэкенд слушает http://127.0.0.1:3000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
