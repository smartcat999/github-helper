FROM rust:1.67 as builder

WORKDIR /usr/src/
COPY config ~/.cargo/config
COPY . .

RUN cargo install --path .


FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/github-helper /usr/local/bin/gctl
COPY ./hack/docker-entrypoint.sh /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]