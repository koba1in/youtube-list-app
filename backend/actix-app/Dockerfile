FROM rust:slim-bookworm AS builder
WORKDIR /app

RUN apt-get update &&\
    apt-get install -y pkg-config libssl-dev &&\
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/base
WORKDIR /app
# RUN adduser usr && chown -R usr /app
# USER usr
COPY --from=builder ./app/target/release/app ./target/release/app

ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT [ "./target/release/app" ]
