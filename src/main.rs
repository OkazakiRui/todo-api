use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

struct AppState {
    counter: Mutex<i32>,
}
async fn index(data: web::Data<AppState>) -> String {
    let counter = data.counter.lock().expect("lock failed");

    format!("current number: {counter}")
}
async fn increment(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().expect("lock failed");
    *counter += 1;

    format!("increment after number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
            .route("/increment", web::get().to(increment))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
