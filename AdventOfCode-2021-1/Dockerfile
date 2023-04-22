FROM rust:1.68 as builder

WORKDIR /usr/src/hello_rust
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/hello_rust/target/release/hello_rust /usr/local/bin/hello_rust
CMD ["hello_rust"]