# URL Shortener Load Tester

A high-performance load testing tool designed to stress test URL shortening services. This tool generates and sends a
large volume of concurrent HTTP requests with randomly generated URLs.

## Features

- Generates random, valid URLs for testing
- Supports high-concurrency testing using Tokio async runtime
- Configurable number of total requests and concurrent tasks
- Measures and reports total execution time

## Requirements

- Rust 1.85.0 or newer
- A running URL shortening service (default endpoint: `http://localhost:8000/shorten`)

## Dependencies

- tokio (1.43) - Asynchronous runtime
- reqwest (0.12) - HTTP client
- rand (0.9.0) - Random data generation

## Configuration

The tool can be configured through constants in the main file:

```rust
const TOTAL_REQUESTS: usize = 1_000_000;    // Total number of requests to send
const CONCURRENT_TASKS: usize = 100;         // Number of parallel tasks
const API_URL: &str = "http://localhost:8000/shorten";  // Target API endpoint
```

## Usage

1. Ensure your URL shortening service is running and accessible
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the tester:
   ```bash
   cargo run --release
   ```

The tool will output the total time taken to complete all requests.

## Performance

The tester is designed for high-performance testing with:

- Concurrent execution using Tokio's async runtime
- Connection pooling through reqwest
- Efficient random URL generation

## Note

This tool is intended for testing purposes only. Please ensure you have permission to perform load testing on the target
service and use responsibly.