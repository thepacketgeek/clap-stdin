#![doc = include_str!("../README.md")]

use std::io;
use std::sync::atomic::AtomicBool;

mod maybe_stdin;
pub use maybe_stdin::MaybeStdin;
mod file_or_stdin;
pub use file_or_stdin::FileOrStdin;

static STDIN_HAS_BEEN_USED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, thiserror::Error)]
pub enum StdinError {
    #[error("stdin argument used more than once")]
    StdInRepeatedUse,
    #[error(transparent)]
    StdIn(#[from] io::Error),
    #[error("unable to parse from_str: {0}")]
    FromStr(String),
}
