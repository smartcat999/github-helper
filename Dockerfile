FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY ./target/${ARCH}-unknown-linux-gnu/github-helper /usr/local/bin/gctl
CMD ["gctl"]