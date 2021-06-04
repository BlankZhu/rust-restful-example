FROM rust:1.52 as builder
WORKDIR /usr/src/api-auth
COPY . .
COPY ./res/config.toml /usr/local/cargo
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/api-auth/target/release/api-auth /usr/local/bin/api-auth
COPY --from=builder /usr/src/api-auth/res/config.yaml /usr/local/conf/config.yaml
ENTRYPOINT ["api-auth", "-c", "/usr/local/conf/config.yaml"]

# docker build -t api-auth:0.1 --network host .
# docker run -d --name api-auth --network dev-mongo -p 8084:8084 api-auth:0.1