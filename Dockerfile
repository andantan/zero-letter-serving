FROM rust:1.90
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN cargo build --release

ENTRYPOINT ["./target/release/zero"]