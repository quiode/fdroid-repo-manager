FROM mcr.microsoft.com/devcontainers/rust
RUN apt-get update && apt-get install -y fdroidserver nginx && rm -rf /var/lib/apt/lists/* 
RUN cargo install cargo-watch

# Fdroid Setup
WORKDIR /fdroid
VOLUME [ "/fdroid" ]
RUN fdroid init

# API Setup
EXPOSE 80
ENV RM_PORT=80
ENV RUST_LOG=debug
ENV RM_IP=0.0.0.0
ENV ANDROID_HOME=/usr/lib/android-sdk

# Dev Container Setup
RUN chown -R vscode:vscode /usr/local/cargo