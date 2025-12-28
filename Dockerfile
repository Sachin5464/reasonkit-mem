# ReasonKit Memory - Multi-stage Docker Build
# Memory & Retrieval Infrastructure

FROM rust:1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release && \
    strip target/release/libreasontkit_mem.so 2>/dev/null || true

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /build/target/release/*.so /app/ 2>/dev/null || true
COPY --from=builder /build/target/release/reasonkit-mem* /app/ 2>/dev/null || true

ENV RUST_LOG=info
EXPOSE 6334
LABEL org.opencontainers.image.source="https://github.com/ReasonKit/reasonkit-mem"
LABEL org.opencontainers.image.description="ReasonKit Memory Infrastructure"

CMD ["/bin/sh", "-c", "echo 'ReasonKit-Mem library ready'"]
