#![allow(dead_code)]

//! An example CLI that uses `FileOrStdin` for a serde_json deserializable value
//! from a file or stdin
//!
//! Example usage:
//! ```sh
//! # via stdin
//! $ echo '{ "name": "Trinity", "age": 30 }' | cargo run --example parse_with_serde -- -
//!
//! # via file read
//! $ cat contents.json
//! '{ "name": "Trinity", "age": 30 }'
//! $ cargo run --example parse_with_serde -- ./contents.json
//! ```
use std::str::FromStr;

use clap::Parser;
use clap_stdin::FileOrStdin;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct User {
    name: String,
    age: u8,
}

impl FromStr for User {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// Parsed user from a file or stdin json
    user: FileOrStdin<User>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    eprintln!("{:?}", args.user);
    Ok(())
}
