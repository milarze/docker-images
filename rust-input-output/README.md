# rust-input-output

This image just writes the input value from the file at the
`INPUT_FILE_PATH` to the file path defined in `OUTPUT_FILE_PATH`
environment variable.

## Entrypoint

The entrypoint is the `/bin/rust-input-output` binary.

## API

### Required ENV

1. `OUTPUT_FILE_PATH`: The path to write the input value out to.
2. `INPUT_FILE_PATH`: The path to read the input value from.

## Example

The following command will print the word "HELLOOOOO" to the
`~/rust-input-output/output/output.txt` file.

```shell
docker run -e OUTPUT_FILE_PATH=/mnt/output.txt  -e INPUT_FILE_PATH=/mnt/input.txt -v "$HOME/rust-input-output/output:/mnt" rust-input-output:0.0.1
```

## Image

The image is based on Alpine linux.
