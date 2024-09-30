FROM rust:1.70 AS builder

# Set the working directory in the container
WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --package spring-rs-hello

# Use a smaller base image for the final container
FROM debian:bullseye-slim

# Install OpenSSL - often required for Rust web applications
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/spring-rs-hello .

EXPOSE 8080

# Command to run your application
CMD ["./spring-rs-hello"]
