# cc_bindings_from_rs

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.


## Invoking as a standalone executable

Most `rustc` cmdline parameters should be supported (e.g. `--crate-type`).

Example:

```

$ cat scratch/test.rs
pub fn public_function() {
    private_function()
}

fn private_function() {}

$ bazel run \
    //cc_bindings_from_rs:cc_bindings_from_rs_legacy_toolchain_runner -- \
    --h_out=$(pwd)/test.h -- \
    $(pwd)/test.rs \
    --crate-type=lib \
    --sysroot $(pwd)/third_party/unsupported_toolchains/rust/toolchains/nightly

$ cat bazel-out/scratch/test.h
// Automatically @generated C++ bindings for the following Rust crate:
// test

// List of public functions:
// public_function
```

## Contributing

See
[rs_bindings_from_cc/README.md](../rs_bindings_from_cc/README.md#contributing).