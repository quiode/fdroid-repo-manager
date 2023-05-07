FROM rust:1-bullseye as builder
WORKDIR /usr/src/fdroid-repo-manager
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y fdroidserver nginx && rm -rf /var/lib/apt/lists/*

# Fdroid Setup
WORKDIR /fdroid
RUN fdroid init

# API Setup
COPY --from=builder /usr/local/cargo/bin/fdroid-repo-manager /usr/local/bin/fdroid-repo-manager
EXPOSE 8080 80
HEALTHCHECK CMD (curl -f http://localhost/fdroid/repo && curl -f http://localhost:8080/health) || exit 1
CMD ["fdroid-repo-manager"]
