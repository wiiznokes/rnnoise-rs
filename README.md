Rust binding for the [rnnoise](https://gitlab.xiph.org/xiph/rnnoise) library.

To enable optimization, only `CFLAGS="-march=native"` seems to be working (see https://github.com/rust-lang/cc-rs/issues/1734).

The C code is vendored. The models are downloaded in the build script. This is not ideal, so it would be nice to be able to provide a path to the model we want to use.

Model are hosted in [this website](https://media.xiph.org/rnnoise/models/).
The crate will use the last by default.

The crate could use any commit of the rnnoise repo.