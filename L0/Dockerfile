# Dockerfile
# 1. Use an official Rust image as a builder
FROM rust:latest as builder

# 2. Create a new empty shell project and copy manifests
RUN cargo new --bin app

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# 3. Build dependencies to cache them
RUN cargo build --release
RUN rm -rf src && mkdir src

# 4. Copy the actual source code and build again
COPY ./src ./src
RUN cargo build --release

# 5. Use a minimal base image and copy the built application
FROM ubuntu:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/l0 /usr/local/bin/l0

# 6. Set the startup command to run the Axum server
CMD ["l0"]