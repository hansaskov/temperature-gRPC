# syntax=docker/dockerfile:1.3-labs

FROM rust:1-alpine3.19 AS build

# Set environment variables
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Install dependencies
RUN apk add --no-cache musl-dev protobuf-dev protoc

# Set the workdir
WORKDIR /app

# Copy only Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# This step compiles only our dependencies and saves them in a layer
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

# Copy the source code
COPY ./ ./

# Build the application
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    <<EOF
    set -e
    cargo build --bin server --release
    strip target/release/server
EOF

# Use a plain alpine image for the final stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Copy the binary from the build stage
COPY --from=build /app/target/release/server /server

# Set the binary as entrypoint
ENTRYPOINT ["/server"]