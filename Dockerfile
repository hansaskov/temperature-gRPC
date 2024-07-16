# Use the official Rust image as a parent image
FROM rust:1-alpine3.19

# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Install dependencies including protoc
RUN apk add --no-cache musl-dev protobuf-dev protoc

# Set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app

# Do a release build
RUN cargo build --bin server --release
RUN strip target/release/server

# Use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.19

# If needed, install additional dependencies here
RUN apk add --no-cache libgcc

# Copy the binary into the final image
COPY --from=0 /app/target/release/server .

# Set the binary as entrypoint
ENTRYPOINT ["/server"]