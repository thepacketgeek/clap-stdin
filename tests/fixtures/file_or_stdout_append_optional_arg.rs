#[cfg(feature = "test_bin")]
use std::io::Write;

use clap::Parser;

use clap_stdin::{Append, FileOrStdout};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short)]
    value: String,
    #[arg(long, default_value = "-")]
    output: FileOrStdout<Append>,
}

#[cfg(feature = "test_bin")]
fn main() {
    let args = Args::parse();
    let mut writer = args.output.into_writer().unwrap();
    let _ = writeln!(&mut writer, "{}", args.value);
}

#[cfg(feature = "test_bin_tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut writer = args.output.into_async_writer().await?;
    tokio::io::AsyncWriteExt::write_all(&mut writer, &args.value.as_bytes()).await?;
    Ok(())
}
