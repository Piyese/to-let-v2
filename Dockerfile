# Build stage
FROM rust:1.85.0-slim-bullseye as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy main.rs file
RUN rm -f src/main.rs

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Install sqlx-cli for running migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

# Production stage
FROM debian:bullseye-slim

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/to-let-v2 ./to-let-v2

# Copy the migrations directory
COPY --from=builder /app/migrations ./migrations

# Make the binary executable
RUN chmod +x ./to-let-v2

# Set the environment variable for the database URL
ENV DATABASE_URL=$DATABASE_URL

# Run the migrations
RUN ./to-let-v2 sqlx migrate run

# Start the application
CMD ["./to-let-v2"]