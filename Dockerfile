FROM rust:1.64 as builder
WORKDIR /usr/src/logger
RUN apt-get update && apt-get install -y cmake
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssh-dev 
COPY --from=builder /usr/local/cargo/bin/logger /usr/local/bin/logger
CMD ["logger"]