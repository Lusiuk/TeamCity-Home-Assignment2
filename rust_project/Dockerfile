FROM rust:1.70-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .

RUN cargo build --release


FROM alpine:3.18
COPY --from=builder /app/target/release/rust_project /usr/local/bin/
CMD ["rust_project"]