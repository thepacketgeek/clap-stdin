use std::io::{self, Read};
use std::str::FromStr;

use super::{Source, StdinError};

/// Wrapper struct to parse arg values from `stdin`
///
/// `MaybeStdin` can wrap any type that matches the trait bounds for `Arg`: `FromStr` and `Clone`
/// ```rust
/// use std::path::PathBuf;
/// use clap::Parser;
/// use clap_stdin::MaybeStdin;
///
/// #[derive(Debug, Parser)]
/// struct Args {
///     path: MaybeStdin<PathBuf>,
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
pub struct MaybeStdin<T> {
    /// Source of the contents
    pub source: Source,
    inner: T,
}

impl<T> FromStr for MaybeStdin<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Source::from_str(s)?;
        match &source {
            Source::Stdin => {
                let stdin = io::stdin();
                let mut input = String::new();
                stdin.lock().read_to_string(&mut input)?;
                Ok(T::from_str(input.trim_end())
                    .map_err(|e| StdinError::FromStr(format!("{e}")))
                    .map(|val| Self { source, inner: val })?)
            }
            Source::Arg(value) => Ok(T::from_str(value)
                .map_err(|e| StdinError::FromStr(format!("{e}")))
                .map(|val| Self { source, inner: val })?),
        }
    }
}

impl<T> MaybeStdin<T> {
    /// Extract the inner value from the wrapper
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::fmt::Display for MaybeStdin<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::fmt::Debug for MaybeStdin<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::ops::Deref for MaybeStdin<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for MaybeStdin<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
