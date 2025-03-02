# Build stage
FROM rust:1.85 as builder

WORKDIR /usr/src/app
COPY . .

# Build the application with release profile
RUN cargo build --release --bin distributed-id-generator

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/distributed-id-generator .

# Expose the port the service runs on
EXPOSE 8080

# Run the service
# Note: Using 0.0.0.0 instead of 127.0.0.1 to allow external connections
ENV PORT=8080
ENV MACHINE_ID=1
ENV DATACENTER_ID=1

CMD ["./distributed-id-generator"]
