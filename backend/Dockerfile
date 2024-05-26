# Use the official Rust image as a parent image
FROM rust:latest

RUN apt-get update && apt-get install -y protobuf-compiler

# Set the working directory inside the container
WORKDIR /backend

COPY . .

# Build the project
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release
RUN cargo install cargo-watch

# Expose the port on which your server will listen
EXPOSE 8080

# Run your server when the container starts
CMD ["cargo", "watch", "--why", "-x", "build"]