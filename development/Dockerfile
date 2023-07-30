FROM rust:1-bullseye
RUN apt-get update && apt-get install -y fdroidserver nginx aapt && rm -rf /var/lib/apt/lists/* 
RUN cargo install cargo-watch

# Fdroid Setup
WORKDIR /fdroid
VOLUME [ "/fdroid" ]
RUN fdroid init

# API Setup
WORKDIR /app
COPY . .
VOLUME [ "/app" ]
EXPOSE 80
ENV RUST_LOG=debug
ENV RM_IP=0.0.0.0
ENV ANDROID_HOME=/usr/lib/android-sdk
HEALTHCHECK CMD (curl -f http://localhost/fdroid/repo && curl -f http://localhost/health) || exit 1
CMD cargo watch -x run

