use std::marker::PhantomData;
use std::str::FromStr;

#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

use super::{Source, StdinError};

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
///     input: FileOrStdin,
/// }
///
/// # fn main() -> anyhow::Result<()> {
/// if let Ok(args) = Args::try_parse() {
///     println!("input={}", args.input.contents()?);
/// }
/// # Ok(())
/// # }
/// ```
///
/// ```sh
/// $ echo "1 2 3 4" > input.txt
/// $ cat input.txt | ./example -
/// 1 2 3 4
///
/// $ ./example input.txt
/// 1 2 3 4
/// ```
#[derive(Debug, Clone)]
pub struct FileOrStdin<T = String> {
    source: Source,
    _type: PhantomData<T>,
}

impl<T> FileOrStdin<T> {
    /// Was this value read from stdin
    pub fn is_stdin(&self) -> bool {
        matches!(self.source, Source::Stdin)
    }

    /// Was this value read from a file (path passed in from argument values)
    pub fn is_file(&self) -> bool {
        !self.is_stdin()
    }

    /// The value passed to this arg (Either "-" for stdin or a filepath)
    pub fn filename(&self) -> &str {
        match &self.source {
            Source::Stdin => "-",
            Source::Arg(path) => path,
        }
    }

    /// Read the entire contents from the input source, returning T::from_str
    pub fn contents(self) -> Result<T, StdinError>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Display,
    {
        use std::io::Read;
        let mut reader = self.into_reader()?;
        let mut input = String::new();
        let _ = reader.read_to_string(&mut input)?;
        T::from_str(input.trim_end()).map_err(|e| StdinError::FromStr(format!("{e}")))
    }

    /// Create a reader from the source, to allow user flexibility of
    /// how to read and parse (e.g. all at once or in chunks)
    ///
    /// ```no_run
    /// use std::io::Read;
    ///
    /// use clap_stdin::FileOrStdin;
    /// use clap::Parser;
    ///
    /// #[derive(Parser)]
    /// struct Args {
    ///   input: FileOrStdin,
    /// }
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let args = Args::parse();
    /// let mut reader = args.input.into_reader()?;
    /// let mut buf = vec![0;8];
    /// reader.read_exact(&mut buf)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_reader(self) -> Result<impl std::io::Read, StdinError> {
        self.source.into_reader()
    }

    #[cfg(feature = "tokio")]
    /// Read the entire contents from the input source, returning T::from_str
    /// ```rust,no_run
    /// use clap::Parser;
    /// use clap_stdin::FileOrStdin;
    ///
    /// #[derive(Debug, Parser)]
    /// struct Args {
    ///     input: FileOrStdin,
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> anyhow::Result<()> {
    /// let args = Args::parse();
    /// println!("input={}", args.input.contents_async().await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn contents_async(self) -> Result<T, StdinError>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Display,
    {
        let mut reader = self.into_async_reader().await?;
        let mut input = String::new();
        let _ = reader.read_to_string(&mut input).await?;
        T::from_str(input.trim_end()).map_err(|e| StdinError::FromStr(format!("{e}")))
    }

    #[cfg(feature = "tokio")]
    /// Create a reader from the source, to allow user flexibility of
    /// how to read and parse (e.g. all at once or in chunks)
    ///
    /// ```no_run
    /// use std::io::Read;
    /// use tokio::io::AsyncReadExt;
    ///
    /// use clap_stdin::FileOrStdin;
    /// use clap::Parser;
    ///
    /// #[derive(Parser)]
    /// struct Args {
    ///   input: FileOrStdin,
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> anyhow::Result<()> {
    /// let args = Args::parse();
    /// let mut reader = args.input.into_async_reader().await?;
    /// let mut buf = vec![0;8];
    /// reader.read_exact(&mut buf).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn into_async_reader(&self) -> Result<impl tokio::io::AsyncRead, StdinError> {
        let input: std::pin::Pin<Box<dyn tokio::io::AsyncRead + 'static>> = match &self.source {
            Source::Stdin => Box::pin(tokio::io::stdin()),
            Source::Arg(filepath) => {
                let f = tokio::fs::File::open(filepath).await?;
                Box::pin(f)
            }
        };
        Ok(input)
    }
}

impl<T> FromStr for FileOrStdin<T> {
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Source::from_str(s)?;
        Ok(Self {
            source,
            _type: PhantomData,
        })
    }
}

#[test]
fn test_source_methods() {
    let val: FileOrStdin<String> = "-".parse().unwrap();
    assert!(val.is_stdin());
    assert!(!val.is_file());
    assert_eq!(val.filename(), "-");

    let val: FileOrStdin<String> = "/path/to/something".parse().unwrap();
    assert!(val.is_file());
    assert!(!val.is_stdin());
    assert_eq!(val.filename(), "/path/to/something");
}
