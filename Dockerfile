# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Install CMake
RUN apt-get update && apt-get install -y cmake

# Copy the entire current directory into the container at /app
COPY . .

# Build your Rust application using Cargo
RUN cargo build --profile release


# Specify the command to run your application
CMD ["./target/release/rust-pro"]
