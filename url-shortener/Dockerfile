# Build stage
FROM rust:1.85 as builder

WORKDIR /usr/src/app
COPY . .

# Install dependencies for compilation
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Build the application with release profile
RUN cargo build --release --bin url-shortener

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/url-shortener .

# Expose the port the service runs on
EXPOSE 8080

# Run the service
CMD ["./url-shortener"]