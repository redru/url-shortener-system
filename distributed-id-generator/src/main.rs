use actix_web::{App, HttpServer, Responder, get, web};
use distributed_id_generator::IdGenerator;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
struct IdResponse {
    id: u64,
}

#[get("/generate")]
async fn generate_id(generator: web::Data<Mutex<IdGenerator>>) -> impl Responder {
    let mut generator = generator.lock().unwrap();
    let id = generator.generate_id();
    web::Json(IdResponse { id })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let generator = IdGenerator::new(4, 2);
    let generator_data = web::Data::new(Mutex::new(generator));

    println!("Server starting at http://127.0.0.1:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(generator_data.clone())
            .service(generate_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
