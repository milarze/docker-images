# rust-echo

This image just writes the input value passed as the command
to the file path defined in `OUTPUT_FILE_PATH` environment variable.

## Entrypoint

The entrypoint is the `/bin/rust-echo` binary.

## API

### Required ENV

1. `OUTPUT_FILE_PATH`: The path to write the input value out to.

### Commands

Only the first value passed to the command is used.

## Example

The following command will print the word "HELLOOOOO" to the
`~/rust-echo/output/output.txt` file.

```shell
docker run -e OUTPUT_FILE_PATH=/mnt/output.txt -v "$HOME/rust-echo/output:/mnt" rust-echo:0.0.1 HELLOOOOO
```

## Image

The image is based on Alpine linux.
