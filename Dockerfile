FROM rust:1.67 as builder

WORKDIR /usr/src/
COPY . .
ADD config ~/.cargo/
RUN make build-local


FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/target/release/gctl /usr/local/bin/gctl
CMD ["gctl"]