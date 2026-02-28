#![doc = include_str!("../README.md")]

use std::io::{self, Read};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;

mod maybe_stdin;
pub use maybe_stdin::MaybeStdin;
mod file_or_stdin;
pub use file_or_stdin::FileOrStdin;
mod file_or_stdout;
pub use file_or_stdout::FileOrStdout;

mod write_mode {
    /// Trait controlling how [`super::FileOrStdout`] opens files.
    pub trait WriteMode: sealed::Sealed + Clone + std::fmt::Debug {
        #[doc(hidden)]
        fn configure(options: &mut std::fs::OpenOptions) -> &mut std::fs::OpenOptions;

        #[doc(hidden)]
        #[cfg(feature = "tokio")]
        fn configure_tokio(options: &mut tokio::fs::OpenOptions) -> &mut tokio::fs::OpenOptions;
    }

    mod sealed {
        pub trait Sealed {}
        impl Sealed for super::Truncate {}
        impl Sealed for super::Append {}
    }

    /// Truncate the file before writing (default).
    #[derive(Debug, Clone)]
    pub struct Truncate;

    impl WriteMode for Truncate {
        fn configure(options: &mut std::fs::OpenOptions) -> &mut std::fs::OpenOptions {
            options.truncate(true)
        }

        #[cfg(feature = "tokio")]
        fn configure_tokio(options: &mut tokio::fs::OpenOptions) -> &mut tokio::fs::OpenOptions {
            options.truncate(true)
        }
    }

    /// Append to the file instead of overwriting.
    #[derive(Debug, Clone)]
    pub struct Append;

    impl WriteMode for Append {
        fn configure(options: &mut std::fs::OpenOptions) -> &mut std::fs::OpenOptions {
            options.append(true)
        }

        #[cfg(feature = "tokio")]
        fn configure_tokio(options: &mut tokio::fs::OpenOptions) -> &mut tokio::fs::OpenOptions {
            options.append(true)
        }
    }
}

pub use write_mode::{Append, Truncate, WriteMode};

static STDIN_HAS_BEEN_READ: AtomicBool = AtomicBool::new(false);

#[derive(Debug, thiserror::Error)]
pub enum StdinError {
    #[error("stdin read from more than once")]
    StdInRepeatedUse,
    #[error(transparent)]
    StdIn(#[from] io::Error),
    #[error("unable to parse from_str: {0}")]
    FromStr(String),
}

/// Source of the value contents will be either from `stdin` or a CLI arg provided value
#[derive(Clone)]
pub(crate) enum Source {
    Stdin,
    Arg(String),
}

impl Source {
    pub(crate) fn into_reader(self) -> Result<impl std::io::Read, StdinError> {
        let input: Box<dyn std::io::Read + 'static> = match self {
            Source::Stdin => {
                if STDIN_HAS_BEEN_READ.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(StdinError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_READ.store(true, std::sync::atomic::Ordering::SeqCst);
                Box::new(std::io::stdin())
            }
            Source::Arg(filepath) => {
                let f = std::fs::File::open(filepath)?;
                Box::new(f)
            }
        };
        Ok(input)
    }

    pub(crate) fn get_value(self) -> Result<String, StdinError> {
        match self {
            Source::Stdin => {
                if STDIN_HAS_BEEN_READ.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(StdinError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_READ.store(true, std::sync::atomic::Ordering::SeqCst);
                let stdin = io::stdin();
                let mut input = String::new();
                stdin.lock().read_to_string(&mut input)?;
                Ok(input)
            }
            Source::Arg(value) => Ok(value),
        }
    }
}

impl FromStr for Source {
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Stdin),
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

/// Destination of the value contents will be either `stdout` or a CLI arg provided filepath
#[derive(Clone)]
pub(crate) enum Dest {
    Stdout,
    Arg(String),
}

impl Dest {
    pub(crate) fn into_writer_with_mode<M: WriteMode>(
        self,
    ) -> std::io::Result<impl std::io::Write> {
        let output: Box<dyn std::io::Write + 'static> = match self {
            Dest::Stdout => Box::new(std::io::stdout()),
            Dest::Arg(filepath) => {
                let mut opts = std::fs::OpenOptions::new();
                opts.create(true).write(true);
                M::configure(&mut opts);
                Box::new(opts.open(filepath)?)
            }
        };
        Ok(output)
    }
}

impl FromStr for Dest {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Stdout),
            arg => Ok(Self::Arg(arg.to_owned())),
        }
    }
}

impl std::fmt::Debug for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dest::Stdout => write!(f, "stdout"),
            Dest::Arg(path) => path.fmt(f),
        }
    }
}
