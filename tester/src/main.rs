use rand::Rng;
use rand::distr::{Alphanumeric, SampleString};
use reqwest::Client;
use std::time::Instant;
use tokio::task;

#[tokio::main]
async fn main() {
    const TOTAL_REQUESTS: usize = 1_000_000;
    const CONCURRENT_TASKS: usize = 100; // Number of concurrent tasks to send requests in parallel
    const API_URL: &str = "http://localhost:8000/shorten";

    let client = Client::new();
    let mut handles = Vec::new();
    let start = Instant::now();

    // Divide the work into concurrent tasks
    for _ in 0..CONCURRENT_TASKS {
        let client = client.clone();

        // Spawn tasks and distribute requests among them
        let handle = task::spawn(async move {
            for _ in 0..(TOTAL_REQUESTS / CONCURRENT_TASKS) {
                let random_url = generate_random_url();
                let payload = format!("{{\"long_url\":\"{}\"}}", random_url);

                // Send request with raw JSON string
                let _ = client
                    .post(API_URL)
                    .header("Content-Type", "application/json")
                    .body(payload)
                    .send()
                    .await;
            }
        });

        handles.push(handle);
    }

    // Wait for all spawned tasks to finish
    for handle in handles {
        handle.await.unwrap();
    }

    let duration = start.elapsed();
    println!(
        "Finished sending {} requests in {:?}",
        TOTAL_REQUESTS, duration
    );
}

// Helper function to generate random URLs
fn generate_random_url() -> String {
    let mut rng = rand::rng();
    let domain_length = rng.random_range(5..15);
    let path_length = rng.random_range(5..20);

    let domain = Alphanumeric.sample_string(&mut rng, domain_length);
    let path = Alphanumeric.sample_string(&mut rng, path_length);

    format!("https://www.{}.com/{}", domain, path)
}
