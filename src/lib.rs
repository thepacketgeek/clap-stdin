#![doc = include_str!("../README.md")]

use std::io::{self, Read};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

static STDIN_HAS_BEEN_USED: AtomicBool = AtomicBool::new(false);

/// Wrapper struct to parse arg values from `stdin`
///
/// `MaybeStdIn` can wrap any time that matches the trait bounds for `Arg`: `FromStr` and `Clone`
/// ```rust
/// use std::path::PathBuf;
/// use clap::Parser;
/// use clap_stdin::MaybeStdIn;
///
/// #[derive(Debug, Parser)]
/// struct Args {
///     path: MaybeStdIn<PathBuf>,
/// }
///
/// if let Ok(args) = Args::try_parse() {
///     println!("path={}", args.path.display());
/// }
/// ```
///
/// ```sh
/// $ pwd | ./example -
/// /current/working/dir
/// ```
#[derive(Clone)]
pub struct MaybeStdIn<T> {
    inner: T,
}

impl<T> MaybeStdIn<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> FromStr for MaybeStdIn<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = StdInError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => {
                if STDIN_HAS_BEEN_USED.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(StdInError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_USED.store(true, Ordering::SeqCst);
                let stdin = io::stdin();
                let mut input = String::new();
                stdin.lock().read_to_string(&mut input)?;
                Ok(T::from_str(input.trim_end())
                    .map_err(|e| StdInError::FromStr(format!("{e}")))
                    .map(|val| MaybeStdIn::new(val))?)
            }
            other => Ok(T::from_str(other)
                .map_err(|e| StdInError::FromStr(format!("{e}")))
                .map(|val| MaybeStdIn::new(val))?),
        }
    }
}

impl<T> MaybeStdIn<T> {
    /// Extract the inner value from the wrapper
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::fmt::Display for MaybeStdIn<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::fmt::Debug for MaybeStdIn<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::ops::Deref for MaybeStdIn<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for MaybeStdIn<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StdInError {
    #[error("stdin argument used more than once")]
    StdInRepeatedUse,
    #[error(transparent)]
    StdIn(#[from] io::Error),
    #[error("unable to parse from_str: {0}")]
    FromStr(String),
}
