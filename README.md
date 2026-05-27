Rust binding for the [rnnoise](https://gitlab.xiph.org/xiph/rnnoise) library.

The C code is vendored. The crate may use any commit of the rnnoise repository.

Models are hosted on [this website](https://media.xiph.org/rnnoise/models/).
By default, the latest model available at the time of release will be downloaded.

You can configure which model will be used with the `RNNOISE_MODEL` environment variable.

Example: `RNNOISE_MODEL=0a8755f8e2d834eff6a54714ecc7d75f9932e845df35f8b59bc52a7cfe6e8b37`

You can also provide a local model via `RNNOISE_MODEL_PATH`, avoiding network access.
The file must be a `tar.gz`.

Example: `RNNOISE_MODEL_PATH="path/to/model.tar.gz"`

To enable optimization, only `CFLAGS="-march=native"` seems to work (see <https://github.com/rust-lang/cc-rs/issues/1734>).
