# Fdroid Repo Manager

## Environment Variables

- RM_IP
- RM_PORT
- RM_REPO_PATH
- RUST_LOG
- RM_MAX_PAYLOAD_SIZE
- ANDROID_HOME

## Development

### Dependencies

- fdroidserver
- aapt

### Build

```bash
cargo build
```

### Run

```bash
RM_IP=127.0.0.1 RM_PORT=8080 RM_REPO_PATH=/home/quio/GitHub/fdroid-repo-manager/development/fdroid RUST_LOG=DEBUG ANDROID_HOME=/opt/android-sdk cargo run
```

`RM_REPO_PATH` should be your custom path to the fdroid directory
