use std::fs;
use std::io::{self, Read};
use std::str::FromStr;
use std::sync::atomic::Ordering;

use super::{StdinError, STDIN_HAS_BEEN_USED};

/// Wrapper struct to either read in a file or contents from `stdin`
///
/// `FileOrStdin` can wrap any type that matches the trait bounds for `Arg`: `FromStr` and `Clone`
/// ```rust
/// use std::path::PathBuf;
/// use clap::Parser;
/// use clap_stdin::FileOrStdin;
///
/// #[derive(Debug, Parser)]
/// struct Args {
///     contents: FileOrStdin,
/// }
///
/// if let Ok(args) = Args::try_parse() {
///     println!("contents={}", args.contents);
/// }
/// ```
///
/// ```sh
/// $ cat <filename> | ./example -
/// <filename> contents
/// ```
///
/// ```sh
/// $ ./example <filename>
/// <filename> contents
/// ```
#[derive(Clone)]
pub struct FileOrStdin<T = String> {
    inner: T,
}

impl<T> FileOrStdin<T> {
    fn new(s: T) -> Self {
        Self { inner: s }
    }
}

impl<T> FromStr for FileOrStdin<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => {
                if STDIN_HAS_BEEN_USED.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(StdinError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_USED.store(true, Ordering::SeqCst);
                let stdin = io::stdin();
                let mut input = String::new();
                stdin.lock().read_to_string(&mut input)?;
                Ok(T::from_str(input.trim_end())
                    .map_err(|e| StdinError::FromStr(format!("{e}")))
                    .map(|val| FileOrStdin::new(val))?)
            }
            filepath => Ok(T::from_str(&fs::read_to_string(filepath)?)
                .map_err(|e| StdinError::FromStr(format!("{e}")))
                .map(|val| FileOrStdin::new(val))?),
        }
    }
}

impl<T> FileOrStdin<T> {
    /// Extract the inner value from the wrapper
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::fmt::Display for FileOrStdin<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::fmt::Debug for FileOrStdin<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::ops::Deref for FileOrStdin<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for FileOrStdin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
