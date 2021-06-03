FROM rust:1.52.1

# Setup
WORKDIR /udpt
COPY Cargo.* .
COPY src .

# Build
RUN cargo test
RUN cargo build --release

# Artifact
FROM debian:bullseye-slim
COPY --from=0 /src/target/release/udpt /usr/local/bin/udpt
