# URL Shortener Service

A distributed URL shortening service built with Rust, consisting of a main URL shortener application and multiple
distributed ID generators.

## System Architecture

The system consists of the following components:

- **URL Shortener Service**: Main application that handles URL shortening requests
- **Distributed ID Generators**: Three instances of ID generators for creating unique identifiers
- **PostgreSQL Database**: Stores the shortened URLs and their mappings

### Components

1. **URL Shortener Service**
    - Main service running on port 8000
    - Handles URL shortening and redirection requests
    - Communicates with distributed ID generators
    - Stores URL mappings in PostgreSQL

2. **Distributed ID Generators**
    - Three independent instances for redundancy and load distribution
    - Running on ports 8080, 8081, and 8082
    - Each configured with unique machine and datacenter IDs
    - Generate unique, distributed IDs for URL shortening

3. **PostgreSQL Database**
    - Runs on port 5433
    - Stores URL mappings and related data
    - Initialized with custom schema using init.sql

## Prerequisites

- Docker
- Docker Compose

## Getting Started

1. Clone the repository:

```bash
git clone <repository-url>
cd <project-directory>
```

2. Start the services:

```bash
docker compose up -d
```

3. Check if all services are running:

```bash
docker compose ps
```

## Service Configuration

### URL Shortener Service

- Port: 8000 (external) -> 8080 (internal)
- Environment variables:
    - Database configuration
    - ID Generator endpoints

### ID Generators

- Generator 1: Port 8080 (Machine ID: 0, Datacenter ID: 1)
- Generator 2: Port 8081 (Machine ID: 1, Datacenter ID: 1)
- Generator 3: Port 8082 (Machine ID: 2, Datacenter ID: 2)

### PostgreSQL

- Port: 5433 (external) -> 5432 (internal)
- Database: url_shortener
- User: url_shortener
- Password: url_shortener
- Persistent volume: url_shortener_postgres_data
- Initialization script: ./url-shortener/sql/init.sql

## Development

### Building Individual Services

To build a specific service:

```bash
docker compose build <service-name>
```

Available services:

- url-shortener
- generator1
- generator2
- generator3

## Maintenance

### Viewing Logs

```bash
docker compose logs -f [service-name]
```

### Stopping Services

```bash
docker compose down
```

To remove volumes when stopping:

```bash
docker compose down -v
```

### Scaling ID Generators

The current setup includes three ID generators. Modify the docker-compose.yaml file to add or remove generators as
needed.

## Notes

- The URL shortener service waits for PostgreSQL to be healthy before starting
- ID generators are configured with unique machine and datacenter IDs to prevent collisions
- PostgreSQL data is persisted using a named volume
- The database is automatically initialized with the schema defined in init.sql

## Dependencies

This project uses:

- PostgreSQL 17
- Rust-based microservices
    - URL Shortener service
    - Distributed ID Generator service