# clap-stdin [![Build](https://img.shields.io/github/actions/workflow/status/thepacketgeek/clap-stdin/ci-build.yml?branch=main)](https://github.com/thepacketgeek/clap-stdin/actions/workflows/ci-build.yml)

This library offers a wrapper type for [`clap`](https://docs.rs/clap) `Arg`s that can
either be passed via CLI argument (positional or optional) or read in from `stdin`. When
an `Arg` value is to be read from `stdin`, the user will pass the commonly used `stdin` alias: `-`

Example usage with `clap`'s `derive` feature for a positional argument:
```rust,no_run
use clap::Parser;

use clap_stdin::MaybeStdIn;

#[derive(Debug, Parser)]
struct Args {
    value: MaybeStdIn<String>,
}

let args = Args::parse();
println!("value={}", args.value);
```

Calling this CLI:
```sh
# using stdin for positional arg value
$ echo "testing" | cargo run -- -
value=testing
```

## Compatible Types
[`MaybeStdIn`] can wrap any time that matches the trait bounds for `Arg`: `FromStr` and `Clone`
```rust
use std::path::PathBuf;
use clap::Parser;
use clap_stdin::MaybeStdIn;

#[derive(Debug, Parser)]
struct Args {
    path: MaybeStdIn<PathBuf>,
}
```

```sh
$ pwd | ./example -
```

## Using `MaybeStdIn` multiple times
[`MaybeStdIn`] will check at runtime if `stdin` is being read from multiple times. You can use this
as a feature if you have mutually exclusive args that should both be able to read from stdin, but know
that the user will receive an error if 2+ `MaybeStdIn` args receive the "-" value.

For example, this compiles:
```rust
use clap_stdin::MaybeStdIn;

#[derive(Debug, clap::Parser)]
struct Args {
    first: MaybeStdIn<String>,
    second: MaybeStdIn<u32>,
}
```

and it will work fine if the stdin alias `-` is only passed for one of the arguments:
```sh
$ echo "2" | ./example FIRST -
```

But if `stdin` is attempted to be used for both arguments, there will be no value for the `second` arg
```sh
$ echo "2" | ./example - -
error: invalid value '-' for '<SECOND>': stdin argument used more than once
```

# License

`clap-stdin` is both MIT and Apache License, Version 2.0 licensed, as found
in the LICENSE-MIT and LICENSE-APACHE files.