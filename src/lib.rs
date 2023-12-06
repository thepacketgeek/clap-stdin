#![doc = include_str!("../README.md")]

use std::io;
use std::str::FromStr;
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

/// Source of the value contents will be either from `stdin` or a CLI arg provided value
#[derive(Clone)]
pub enum Source {
    Stdin,
    Arg(String),
}

impl FromStr for Source {
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => {
                if STDIN_HAS_BEEN_USED.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(StdinError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_USED.store(true, std::sync::atomic::Ordering::SeqCst);
                Ok(Self::Stdin)
            }
            arg => Ok(Self::Arg(arg.to_owned())),
        }
    }
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Stdin => write!(f, "stdin"),
            Source::Arg(v) => v.fmt(f),
        }
    }
}
