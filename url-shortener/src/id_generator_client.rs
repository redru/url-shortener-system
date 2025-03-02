use serde::Deserialize;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Deserialize)]
pub struct GeneratorResponse {
    pub id: i64,
}

pub struct IdGeneratorClient {
    clients: Vec<String>,
    current: AtomicUsize,
    http_client: reqwest::Client,
}

impl IdGeneratorClient {
    pub fn new() -> Self {
        // Initialize with the three generator endpoints from docker-compose
        let clients = std::env::var("GENERATOR_URLS")
            .unwrap_or_else(|_| {
                "http://localhost:8080,http://localhost:8081,http://localhost:8082".to_string()
            })
            .split(',')
            .map(|s| s.to_string())
            .collect();

        Self {
            clients,
            current: AtomicUsize::new(0),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn generate_id(&self) -> Result<i64, reqwest::Error> {
        // Get next index using atomic operation for thread safety
        let current_index = self.current.fetch_add(1, Ordering::SeqCst) % self.clients.len();

        // Create the full URL for the current generator
        let url = format!("{}/generate", self.clients[current_index]);

        // Make the request
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .json::<GeneratorResponse>()
            .await?;

        Ok(response.id)
    }
}
