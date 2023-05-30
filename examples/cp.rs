//! An example CLI replicating the functionality of the `cp` utility
//! that accepts a source path from stdin
//!
//! Example usage:
//! ```sh
//! $ find . -name Cargo.toml | cargo run --example cp -- - ./Cargo.toml.bak
//! ```
use std::path::PathBuf;

use clap::Parser;
use clap_stdin::MaybeStdin;

#[derive(Debug, Parser)]
struct Args {
    /// source file to copy from (use "-" to pass from stdin)
    source: MaybeStdin<PathBuf>,
    /// new destination file path
    dest: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let source = args.source.into_inner();
    let dest = args.dest;

    eprintln!("Copy from '{}' to '{}'", source.display(), dest.display());
    std::fs::copy(&source, &dest)?;
    Ok(())
}
