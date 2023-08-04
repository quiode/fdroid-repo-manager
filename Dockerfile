FROM rust:1-bullseye as builder
COPY ./backend /usr/src/fdroid-repo-manager/backend
WORKDIR /usr/src/fdroid-repo-manager/backend
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y fdroidserver nginx aapt && rm -rf /var/lib/apt/lists/*

# Fdroid Setup
WORKDIR /fdroid
VOLUME [ "/fdroid" ]
RUN fdroid init

# API Setup
COPY --from=builder /usr/local/cargo/bin/fdroid-repo-manager /usr/local/bin/fdroid-repo-manager
ENV RM_IP=0.0.0.0
ENV ANDROID_HOME=/usr/lib/android-sdk
HEALTHCHECK CMD (curl -f http://localhost/fdroid/repo && curl -f http://localhost/health) || exit 1
EXPOSE 80
CMD ["fdroid-repo-manager"]
