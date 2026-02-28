use std::marker::PhantomData;
use std::str::FromStr;

use super::{Dest, Truncate, WriteMode};

/// `FileOrStdout` can be used as a proxy output writer to write to whichever destination
/// was specified by the CLI args, a file or `stdout`.
///
/// The type parameter `M` controls how the file is opened:
/// - [`Truncate`] (default): truncates the file before writing
/// - [`Append`](super::Append): appends to the file
///
/// ```rust
/// use std::path::PathBuf;
/// use std::io::Write;
/// use clap::Parser;
/// use clap_stdin::FileOrStdout;
///
/// #[derive(Debug, Parser)]
/// struct Args {
///     output: FileOrStdout,
/// }
///
/// # fn main() -> anyhow::Result<()> {
/// if let Ok(args) = Args::try_parse() {
///     let mut writer = args.output.into_writer()?;
///     write!(&mut writer, "1 2 3 4");
/// }
/// # Ok(())
/// # }
/// ```
///
/// ```sh
/// $ ./example output.txt
/// 1 2 3 4
/// $ cat output.txt | ./example -
/// 1 2 3 4
/// ```
#[derive(Debug, Clone)]
pub struct FileOrStdout<M: WriteMode = Truncate> {
    dest: Dest,
    _mode: PhantomData<M>,
}

impl<M: WriteMode> FileOrStdout<M> {
    /// Was this value read from stdout
    pub fn is_stdout(&self) -> bool {
        matches!(self.dest, Dest::Stdout)
    }

    /// Was this value read from a file (path passed in from argument values)
    pub fn is_file(&self) -> bool {
        !self.is_stdout()
    }

    /// The value passed to this arg (Either "-" for stdout or a filepath)
    pub fn filename(&self) -> &str {
        match &self.dest {
            Dest::Stdout => "-",
            Dest::Arg(path) => path,
        }
    }

    /// Create a writer for the dest, to allow user flexibility of
    /// how to write output (e.g. all at once or in chunks)
    ///
    /// ```no_run
    /// use std::io::Write;
    ///
    /// use clap_stdin::FileOrStdout;
    /// use clap::Parser;
    ///
    /// #[derive(Parser)]
    /// struct Args {
    ///   output: FileOrStdout,
    /// }
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let args = Args::parse();
    /// let mut writer = args.output.into_writer()?;
    /// let mut buf = vec![0;8];
    /// writer.write_all(&mut buf)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_writer(self) -> Result<impl std::io::Write, std::io::Error> {
        self.dest.into_writer_with_mode::<M>()
    }

    #[cfg(feature = "tokio")]
    /// Create a writer from the dest, to allow user flexibility of
    /// how to write output (e.g. all at once or in chunks)
    ///
    /// ```no_run
    /// use std::io::Write;
    /// use tokio::io::AsyncWriteExt;
    ///
    /// use clap_stdin::FileOrStdout;
    /// use clap::Parser;
    ///
    /// #[derive(Parser)]
    /// struct Args {
    ///   output: FileOrStdout,
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> anyhow::Result<()> {
    /// let args = Args::parse();
    /// let mut writer = args.output.into_async_writer().await?;
    /// let mut buf = vec![0;8];
    /// writer.write_all(&mut buf).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn into_async_writer(&self) -> std::io::Result<impl tokio::io::AsyncWrite> {
        let output: std::pin::Pin<Box<dyn tokio::io::AsyncWrite + 'static>> = match &self.dest {
            Dest::Stdout => Box::pin(tokio::io::stdout()),
            Dest::Arg(filepath) => {
                let mut opts = tokio::fs::OpenOptions::new();
                opts.create(true).write(true);
                M::configure_tokio(&mut opts);
                let f = opts.open(filepath).await?;
                Box::pin(f)
            }
        };
        Ok(output)
    }
}

impl<M: WriteMode> FromStr for FileOrStdout<M> {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dest = Dest::from_str(s)?;
        Ok(Self {
            dest,
            _mode: PhantomData,
        })
    }
}

#[test]
fn test_source_methods() {
    let val: FileOrStdout = "-".parse().unwrap();
    assert!(val.is_stdout());
    assert!(!val.is_file());
    assert_eq!(val.filename(), "-");

    let val: FileOrStdout = "/path/to/something".parse().unwrap();
    assert!(val.is_file());
    assert!(!val.is_stdout());
    assert_eq!(val.filename(), "/path/to/something");
}
