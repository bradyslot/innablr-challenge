FROM rust:latest as builder
WORKDIR /usr/src/innablr-challenge
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/src/innablr-challenge/target/release/innablr-challenge /usr/local/bin/innablr-challenge
EXPOSE 8000/tcp
ENTRYPOINT ["innablr-challenge"]
