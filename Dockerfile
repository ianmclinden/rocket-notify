FROM rust:alpine AS builder
WORKDIR /usr/src/rocket-notify
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo install --locked --path .

FROM alpine:latest
LABEL org.opencontainers.image.authors="Ian McLinden"
COPY --from=builder /usr/local/cargo/bin/rocket-notify /usr/local/bin/rocket-notify
RUN mkdir -p /config
WORKDIR /config

ENTRYPOINT [ "/usr/local/bin/rocket-notify" ]