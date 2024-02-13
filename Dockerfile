FROM rust:slim-buster AS build
WORKDIR /app
COPY . .
RUN cargo build --release
ENTRYPOINT ["/app/target/release/httpd"]
