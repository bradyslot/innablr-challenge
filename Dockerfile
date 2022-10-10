FROM rust:latest as builder
WORKDIR /usr/src/innablr-challenge
COPY . .
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /usr/src/innablr-challenge/target/release/innablr-challenge /bin
EXPOSE 8000/tcp
ENTRYPOINT ["innablr-challenge"]
