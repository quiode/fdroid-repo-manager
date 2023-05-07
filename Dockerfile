FROM rust:1-bullseye as builder
WORKDIR /usr/src/fdroid-repo-manager
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/fdroid-repo-manager /usr/local/bin/fdroid-repo-manager
CMD ["fdroid-repo-manager"]
