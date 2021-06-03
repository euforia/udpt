FROM rust:1.52.1

# Setup
WORKDIR /udpt
COPY . .

# Build
RUN cargo build --release

# Artifact
FROM debian:bullseye-slim
COPY --from=0 /udpt/target/release/udpt /usr/local/bin/udpt
