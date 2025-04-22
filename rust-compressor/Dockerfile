# Stage 1: Build statically linked binary with MUSL
FROM rust:latest AS builder


WORKDIR /app

# Enable MUSL target
RUN rustup target add x86_64-unknown-linux-musl

# Copy project files
COPY Cargo.toml .
COPY src ./src

# Build release binary statically
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage: scratch image
FROM scratch

# Copy only the binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-compressor /rust-compressor

ENTRYPOINT ["/rust-compressor"]
