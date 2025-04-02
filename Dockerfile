# Use a Rust base image with Cargo installed
FROM docker.io/rust:slim-bookworm AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to trick Cargo into thinking it's a valid Rust project
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies without the actual source code to cache dependencies separately
RUN apt update && apt install default-libmysqlclient-dev libpq-dev libsqlite3-dev musl-tools -y \
  && rustup target add x86_64-unknown-linux-musl 

RUN cargo build --release


# Now copy the source code
COPY ./src ./src

# Build your application
RUN cargo build --release


# Start a new stage to create a smaller image without unnecessary build dependencies
FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libmariadb3 libpq5 sqlite3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd -m rustuser
USER rustuser

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/axum ./

# Command to run the application
CMD ["/app/axum"]
