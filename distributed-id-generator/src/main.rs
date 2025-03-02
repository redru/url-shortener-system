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
    let hostname = "0.0.0.0";
    let port = std::env::var("PORT").map_or_else(|_| 8080, |v| v.parse().unwrap());
    let machine_id = std::env::var("MACHINE_ID").map_or_else(
        |_| 1,
        |v| v.parse::<u64>().expect("MACHINE_ID must be a valid number"),
    );

    let datacenter_id = std::env::var("DATACENTER_ID").map_or_else(
        |_| 1,
        |v| {
            v.parse::<u64>()
                .expect("DATACENTER_ID must be a valid number")
        },
    );

    let generator = IdGenerator::new(machine_id, datacenter_id);
    let generator_data = web::Data::new(Mutex::new(generator));

    println!("Server starting at http://{}:{}", hostname, port);

    HttpServer::new(move || {
        App::new()
            .app_data(generator_data.clone())
            .service(generate_id)
    })
    .bind((hostname, port))?
    .run()
    .await
}
