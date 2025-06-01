### Build ###
FROM rust:1.85.1-slim AS builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new minecraft-server

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/minecraft-server/

WORKDIR /usr/src/minecraft-server

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/minecraft-server/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/minecraft-server/src/bin/start-server/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

### Runtime ###
FROM alpine:3.16.0 AS runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/minecraft-server/target/x86_64-unknown-linux-musl/release/start-server /usr/local/bin

EXPOSE 25565

ENTRYPOINT ["/usr/local/bin/start-server"]