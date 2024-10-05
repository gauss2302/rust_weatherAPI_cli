# Use the official Rust image as a parent image
FROM rust:1.75-slim-buster as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Build the application
RUN cargo build --release

# Start a new stage for a smaller final image
FROM debian:buster-slim

# Install OpenSSL
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/weather_cli_project /usr/local/bin/weather_cli_project

# Set the startup command to run your binary
CMD ["weather_cli_project"]