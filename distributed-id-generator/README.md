# Distributed ID Generator

A high-performance, distributed unique ID generator service implemented in Rust, inspired by Twitter's Snowflake ID
generation system. This service generates unique 64-bit IDs that are time-sortable and guaranteed to be unique across
multiple nodes in a distributed system.

## Features

- **Distributed Architecture**: Supports multiple instances across different datacenters and machines
- **Time-sortable IDs**: Generated IDs preserve temporal ordering

## ID Structure

The generated 64-bit IDs are composed of the following components:

- Timestamp (41 bits): Milliseconds since epoch
- Datacenter ID (5 bits): Supports up to 32 datacenters
- Machine ID (5 bits): Supports up to 32 machines per datacenter
- Sequence number (12 bits): Allows for 4096 IDs per millisecond per machine

## API

### Generate ID

Returns a JSON response containing a unique ID:

```json
{
  "id": 1234567890
}
```

## Configuration

The service can be configured using environment variables:

- `PORT`: Server port (default: 8080)
- `MACHINE_ID`: Unique machine identifier (0-31)
- `DATACENTER_ID`: Datacenter identifier (0-31)

## Deployment

### Docker

Build and run using Docker:

```bash
docker build -t distributed-id-generator .
docker run -p 8080:8080 \
    -e MACHINE_ID=1 \
    -e DATACENTER_ID=1 \
    distributed-id-generator
```

### Docker Compose

The service can be deployed as part of a larger system using Docker Compose. The provided configuration demonstrates
running multiple generator instances:

```bash
docker-compose up
```

This will start:

- Three ID generator instances with different machine IDs
- Each instance exposed on different ports (8080, 8081, 8082)
- Configured with different datacenter and machine IDs for distributed operation

## Technical Details

### Clock Synchronization

The system uses a monotonic clock implementation to ensure time-based ordering of IDs. The clock system:

- Handles time synchronization across nodes
- Provides millisecond-precision timestamps
- Includes protection against clock drift

### Sequence Generation

- Each node maintains a sequence counter
- Resets every second
- Supports up to 4096 unique IDs per second per node

## Dependencies

- `actix-web`: Web framework for the HTTP API
- `serde`: Serialization/deserialization framework
- Other standard Rust libraries for timing and synchronization

## Building from Source

```bash
# Clone the repository
git clone <repository-url>

# Build the project
cargo build --release

# Run the tests
cargo test

# Run the service
cargo run --release
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests.

## License

[Insert appropriate license information here]