# Duas fases: compila com a toolchain inteira, roda numa imagem minima.
FROM docker.io/library/rust:1.82-slim AS builder
WORKDIR /src
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim
WORKDIR /app
COPY --from=builder /src/target/release/loja-api /app/loja-api
EXPOSE 8080
CMD ["/app/loja-api"]
