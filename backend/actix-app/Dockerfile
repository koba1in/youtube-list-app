# FROM rust:slim-bookworm AS builder
# WORKDIR /app

# RUN apt-get update &&\
#     apt-get install -y pkg-config libssl-dev &&\
#     rm -rf /var/lib/apt/lists/*

# COPY Cargo.toml Cargo.lock ./
# RUN mkdir src && echo "fn main() {}" > src/main.rs
# RUN cargo build --release

# COPY . .
# RUN cargo build --release

# FROM gcr.io/distroless/base
# WORKDIR /app
# # RUN adduser usr && chown -R usr /app
# # USER usr
# COPY --from=builder ./app/target/release/app ./target/release/app

# ENV PORT=8080
# EXPOSE ${PORT}
# ENTRYPOINT [ "./target/release/app" ]

FROM rust:slim-bookworm

# 依存キャッシュを効かせるため、まず Cargo.toml/Cargo.lock だけをコピー
WORKDIR /app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

# ダミー main.rs を作って deps を pre-build（キャッシュ）
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build

# 本物のコードをコピー
COPY . .

# ホスト側と同期して開発する場合は CMD は任意
CMD ["cargo", "run"]
