FROM rust:latest as builder
WORKDIR /usr/src/innablr-challenge
COPY . .
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN cargo build --release
RUN cargo install --path . --verbose

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/innablr-challenge /bin
EXPOSE 8000/tcp
ENTRYPOINT ["innablr-challenge"]
