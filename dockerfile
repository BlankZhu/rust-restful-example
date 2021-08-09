FROM rust:1.54 as builder
WORKDIR /usr/src/rust-restful-example
COPY . .
COPY ./res/config.toml /usr/local/cargo
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/rust-restful-example/target/release/rust-restful-example /usr/local/bin/rust-restful-example
COPY --from=builder /usr/src/rust-restful-example/res/config.yaml /usr/local/conf/config.yaml
ENTRYPOINT ["rust-restful-example", "-c", "/usr/local/conf/config.yaml"]

# docker build -t rust-restful-example:0.1 --network host .
# docker run -d --name rust-restful-example --network dev-mongo -p 8084:8084 rust-restful-example:0.1