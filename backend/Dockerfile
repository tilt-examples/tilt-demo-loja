# Duas fases: compila com a toolchain inteira, roda numa imagem minima.
FROM docker.io/library/rust:1.85-slim AS builder
WORKDIR /src
# O Cargo.lock fixa as versões: sem ele, cada build resolvia as mais novas do dia, e em 29/09 o
# hyper-util 0.1.21 passou a pedir edition 2024 (Rust 1.85) e o build quebrou sem mudança nossa.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim
WORKDIR /app
COPY --from=builder /src/target/release/loja-api /app/loja-api
EXPOSE 8080
CMD ["/app/loja-api"]
