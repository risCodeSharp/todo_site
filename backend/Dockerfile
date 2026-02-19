# ----------------------
# 1) Build Stage
# ----------------------

# Use the latest Rust official image
FROM rust:latest AS builder

# Set working directory
WORKDIR /usr/src/app

# Copy dependency definitions
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs so cargo can cache deps
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy file
RUN rm -rf src

# Now copy the real source code
COPY . .

# Build the real project in release mode
RUN cargo build --release

# ----------------------
# 2) Runtime Stage
# ----------------------

# Use a minimal base for runtime
FROM debian:bookworm-slim

# Install PostgreSQL client library (needed by sqlx)
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set workdir
WORKDIR /app

# Copy the compiled binary from the builder
COPY --from=builder /usr/src/app/target/release/todo_backend .

# (Optional) Copy migrations if you need to run them inside the container
COPY --from=builder /usr/src/app/migrations ./migrations

# Expose the port your server listens on
EXPOSE 6767

# Start the server
CMD ["./todo_backend"]
