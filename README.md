# Fdroid Repo Manager

## Setup

### Docker

TODO: when frontend is done

## Environment Variables

- RM_IP  
  IP Adress of the server
- RM_PORT  
  Port of the server
- RM_REPO_PATH  
  Path to the repository
- RUST_LOG  
  Log level of the logger
- RM_MAX_PAYLOAD_SIZE  
  Max Payload size (important for apk uploads)
- ANDROID_HOME  
  path to the android sdk

## Routes

- `/`  
frontend
- `/api`  
backend
- `/fdroid`  
fdroid repository

## Development

### Dependencies

- fdroidserver
- aapt

### Backend

Backend is inside `./backend`

#### Build

```bash
cargo build
```

#### Run

```bash
RM_IP=127.0.0.1 RM_PORT=8080 RM_REPO_PATH=/home/quio/GitHub/fdroid-repo-manager/development/fdroid RUST_LOG=DEBUG ANDROID_HOME=/opt/android-sdk cargo run
```

`RM_REPO_PATH` should be your custom path to the fdroid directory
