FROM rust:1-bullseye as backend-builder
COPY ./backend /usr/src/fdroid-repo-manager/backend/
WORKDIR /usr/src/fdroid-repo-manager/backend/
RUN cargo install --path .

FROM node:18-alpine as frontend-builder
# Install dependencies
RUN npm i -g pnpm
COPY ./frontend/package.json /usr/src/fdroid-repo-manager/frontend/
COPY ./frontend/pnpm-lock.yaml /usr/src/fdroid-repo-manager/frontend/
WORKDIR /usr/src/fdroid-repo-manager/frontend/
RUN pnpm install
# Build
COPY ./frontend /usr/src/fdroid-repo-manager/frontend/
RUN pnpm build

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y fdroidserver nginx aapt && rm -rf /var/lib/apt/lists/*

# Fdroid Setup
WORKDIR /fdroid
VOLUME [ "/fdroid" ]
RUN fdroid init

# Frontend Setup
COPY --from=frontend-builder /usr/src/fdroid-repo-manager/frontend/dist/ /website

# API Setup
COPY --from=backend-builder /usr/local/cargo/bin/fdroid-repo-manager /usr/local/bin/fdroid-repo-manager
ENV RM_IP=0.0.0.0
ENV ANDROID_HOME=/usr/lib/android-sdk
ENV RM_PORT=80
EXPOSE 80

HEALTHCHECK CMD (curl -f http://localhost/fdroid/repo && curl -f http://localhost/api/health && curl -f http://localhost) || exit 1
CMD ["fdroid-repo-manager"]
