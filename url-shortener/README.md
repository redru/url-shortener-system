# URL Shortener Service

A robust URL shortening service built with Rust, featuring a distributed ID generation system and PostgreSQL storage.

## Features

- URL shortening with base62 encoding
- Distributed ID generation across multiple nodes
- PostgreSQL-based storage
- RESTful API endpoints
- Docker support for easy deployment

## Prerequisites

- Rust 1.85 or later
- PostgreSQL database
- Docker (optional, for containerized deployment)

## Environment Variables

The service can be configured using the following environment variables:

```env
PORT=8080                     # Service port (default: 8080)
POSTGRES_USER=url_shortener   # Database user
POSTGRES_PASSWORD=url_shortener # Database password
POSTGRES_HOST=localhost       # Database host
POSTGRES_PORT=5433           # Database port
POSTGRES_DB=url_shortener    # Database name
GENERATOR_URLS=http://localhost:8080,http://localhost:8081,http://localhost:8082  # ID generator endpoints
```

## API Endpoints

### 1. Shorten URL

```http
POST /shorten
Content-Type: application/json

{
    "long_url": "https://example.com/very/long/url"
}
```

Response:

```json
{
  "short_url": "abc123"
}
```

### 2. Redirect to Original URL

```http
GET /{short_url}
```

Redirects to the original URL if found, returns 404 if not found.

## Building and Running

### Local Development

1. Clone the repository
2. Set up the PostgreSQL database
3. Run the service:

```bash
cargo run
```

### Docker Deployment

Build the container:

```bash
docker build -t url-shortener .
```

Run the container:

```bash
docker run -p 8080:8080 \
  -e POSTGRES_USER=url_shortener \
  -e POSTGRES_PASSWORD=url_shortener \
  -e POSTGRES_HOST=db \
  -e POSTGRES_PORT=5433 \
  -e POSTGRES_DB=url_shortener \
  url-shortener
```

## Technical Details

- Built with Actix-web framework
- Uses SQLx for type-safe database operations
- Implements a round-robin ID generator client for distributed ID generation
- Uses base62 encoding for generating human-readable short URLs
- Supports high availability through multiple ID generator instances

## Project Structure

- `main.rs`: Application entry point and API handlers
- `lib.rs`: Module declarations
- `shortener_service.rs`: Core URL shortening logic and database operations
- `id_generator_client.rs`: Distributed ID generation client
- `Dockerfile`: Multi-stage build configuration for containerization

## Dependencies

- actix-web 4.9: Web framework
- base62 2.2: URL-safe encoding
- reqwest 0.12: HTTP client
- serde 1.0: Serialization framework
- sqlx 0.8: Async PostgreSQL client

## License

[Add your chosen license here]

## Contributing

[Add contribution guidelines here]