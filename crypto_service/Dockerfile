# Use the official Rust image as a parent image
FROM rust:latest

RUN apt update
RUN apt install -y protobuf-compiler

# Set the working directory inside the container
WORKDIR /crypto_service

COPY . .

# Build the project
RUN cargo build --release


# Expose the port on which your server will listen
EXPOSE 50051

# Run your server when the container starts
CMD ["cargo", "run", "--release"]


