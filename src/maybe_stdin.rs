use std::io::{self, Read};
use std::str::FromStr;
use std::sync::atomic::Ordering;

use super::{StdinError, STDIN_HAS_BEEN_USED};

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
    inner: T,
}

impl<T> MaybeStdin<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> FromStr for MaybeStdin<T>
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
                    .map(|val| MaybeStdin::new(val))?)
            }
            other => Ok(T::from_str(other)
                .map_err(|e| StdinError::FromStr(format!("{e}")))
                .map(|val| MaybeStdin::new(val))?),
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
