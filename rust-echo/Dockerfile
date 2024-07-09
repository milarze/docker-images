FROM rust:latest as build

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rust-echo /bin/rust-echo

ENTRYPOINT ["/bin/rust-echo"]

