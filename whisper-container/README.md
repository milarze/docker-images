# Whisper Container

Build an image with the packaged Whisper model and expose a CLI via the `ENTRYPOINT`.

## Building

```shell
docker build -t whisper-container:latest -f Dockerfile .
```

## Running

```shell
docker run whisper-container:latest --help
```
