use actix_web::{App, HttpResponse, HttpServer, web};
use std::env;
use url_shortener::id_generator_client::IdGeneratorClient;
use url_shortener::shortener_service::ShortenerService;

#[actix_web::get("/{short_url}")]
async fn redirect(
    service: web::Data<ShortenerService>,
    short_url: web::Path<String>,
) -> HttpResponse {
    match service.find_by_short_url(&short_url).await {
        Ok(Some(mapping)) => HttpResponse::Found()
            .append_header(("Location", mapping.long_url))
            .finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
struct ShortenRequest {
    long_url: String,
}

#[derive(serde::Serialize)]
struct ShortenResponse {
    short_url: String,
}

impl ShortenResponse {
    pub fn new(short_url: String) -> Self {
        Self { short_url }
    }
}

#[actix_web::post("/shorten")]
async fn shorten(
    shortener_service: web::Data<ShortenerService>,
    id_generator_client: web::Data<IdGeneratorClient>,
    shorten_request: web::Json<ShortenRequest>,
) -> HttpResponse {
    let shorten_request = shorten_request.into_inner();

    match shortener_service
        .find_by_long_url(&shorten_request.long_url)
        .await
    {
        Ok(Some(existing)) => HttpResponse::Ok().json(ShortenResponse::new(existing.short_url)),
        Ok(None) => match id_generator_client.generate_id().await {
            Ok(generated_id) => {
                let url = shortener_service
                    .insert_url(
                        generated_id,
                        &shorten_request.long_url,
                        &base62::encode(generated_id as u128),
                    )
                    .await
                    .unwrap();
                HttpResponse::Ok().json(ShortenResponse::new(format!(
                    "https://redru.io/{}",
                    url.short_url
                )))
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0";
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Server starting at http://{}:{}", address, port);

    // Initialize the shortener service
    let shortener_service = ShortenerService::new()
        .await
        .expect("Failed to create shortener service");

    let shortener_service_data = web::Data::new(shortener_service);

    let id_generator_client = IdGeneratorClient::new();
    let id_generator_client_data = web::Data::new(id_generator_client);

    HttpServer::new(move || {
        App::new()
            .app_data(shortener_service_data.clone())
            .app_data(id_generator_client_data.clone())
            .service(redirect)
            .service(shorten)
    })
    .bind((address, port))?
    .run()
    .await
}
