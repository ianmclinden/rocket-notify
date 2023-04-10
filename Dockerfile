FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
LABEL org.opencontainers.image.authors="Ian McLinden"
COPY --from=builder /usr/local/cargo/bin/rocket-notify /usr/local/bin/rocket-notify
RUN mkdir -p /config
WORKDIR /config

ENTRYPOINT [ "/usr/local/bin/rocket-notify" ]