use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello actix!!")
}

#[post("/echo")]
async fn echo(body: String) -> impl Responder {
    HttpResponse::Ok().body(body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/manual", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
