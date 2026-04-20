FROM rust:1.75-slim as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/neon-defense-api .
EXPOSE 3000
CMD ["./neon-defense-api"]