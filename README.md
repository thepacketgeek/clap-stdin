# clap-stdin [![Build](https://img.shields.io/github/actions/workflow/status/thepacketgeek/clap-stdin/ci-build.yml?branch=main)](https://github.com/thepacketgeek/clap-stdin/actions/workflows/ci-build.yml)

This library offers two wrapper types for [`clap`](https://docs.rs/clap) `Arg`s that help
for cases where values may be passed in via `stdin`. When an `Arg` value is to be read
from `stdin`, the user will pass the commonly used `stdin` alias: `-`

- `MaybeStdin`: Used when a value can be passed in via args OR `stdin`
- `FileOrStdin`: Used when a value can be read in from a file OR `stdin`

## `MaybeStdin`

Example usage with `clap`'s `derive` feature for a positional argument:
```rust,no_run
use clap::Parser;

use clap_stdin::MaybeStdin;

#[derive(Debug, Parser)]
struct Args {
    value: MaybeStdin<String>,
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
[`MaybeStdin`] can wrap any type that matches the trait bounds for `Arg`: `FromStr` and `Clone`
```rust
use std::path::PathBuf;
use clap::Parser;
use clap_stdin::MaybeStdin;

#[derive(Debug, Parser)]
struct Args {
    path: MaybeStdin<PathBuf>,
}
```

```sh
$ pwd | ./example -
```

## `FileOrStdin`

Example usage with `clap`'s `derive` feature for a positional argument:
```rust,no_run
use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    input: FileOrStdin,
}

# fn main() -> anyhow::Result<()> {
let args = Args::parse();
println!("input={}", args.input.contents()?);
# Ok(())
# }
```

Calling this CLI:
```sh
# using stdin for positional arg value
$ echo "testing" | cargo run -- -
input=testing

# using filename for positional arg value
$ echo "testing" > input.txt
$ cargo run -- input.txt
input=testing
```

## Compatible Types
[`FileOrStdin`] can wrap any type that matches the trait bounds for `Arg`: `FromStr` and `Clone`
```rust
use std::path::PathBuf;
use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    path: FileOrStdin<u32>,
}
```

```sh
# Value from stdin
$ wc ~/myfile.txt -l | ./example -

# Value from file
$ cat myfile.txt
42
$ .example myfile.txt
```

## Reading from Stdin without special characters
When using [`MaybeStdin`] or [`FileOrStdin`], you can allow your users to omit the "-" character to read from `stdin` by providing a `default_value` to clap. This works with positional and optional args:

```rust,no_run
use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    #[clap(default_value = "-")]
    input: FileOrStdin,
}

# fn main() -> anyhow::Result<()> {
let args = Args::parse();
println!("input={}", args.input.contents()?);
# Ok(())
# }
```

Calling this CLI:
```sh
# using stdin for positional arg value
$ echo "testing" | cargo run
input=testing

# using filename for positional arg value
$ echo "testing" > input.txt
$ cargo run -- input.txt
input=testing
```

## Using `MaybeStdin` or `FileOrStdin` multiple times
Both [`MaybeStdin`] and [`FileOrStdin`] will check at runtime if `stdin` is being read from multiple times. You can use this
as a feature if you have mutually exclusive args that should both be able to read from stdin, but know
that the user will receive an error if 2+ `MaybeStdin` args receive the "-" value.

For example, this compiles:
```rust
use clap_stdin::{FileOrStdin, MaybeStdin};

#[derive(Debug, clap::Parser)]
struct Args {
    first: FileOrStdin,
    second: MaybeStdin<u32>,
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