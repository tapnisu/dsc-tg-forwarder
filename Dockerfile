FROM rust:alpine3.22 AS builder
LABEL authors="tapnisu"

WORKDIR /usr/src/dsc-tg-forwarder

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache alpine-sdk libressl-dev

COPY . .
RUN cargo build --release

FROM alpine:3.22 AS runner

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache ca-certificates \
    && update-ca-certificates

COPY --from=builder /usr/src/dsc-tg-forwarder/target/release/dsc-tg-forwarder /usr/local/bin/dsc-tg-forwarder

CMD ["dsc-tg-forwarder"]
