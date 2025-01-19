use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value = "-")]
    first: FileOrStdin,
    #[arg(short, long)]
    second: Option<String>,
}

#[cfg(feature = "test_bin")]
fn main() -> Result<(), String> {
    let args = Args::parse();
    println!(
        "FIRST: {}; SECOND: {:?}",
        args.first.contents().map_err(|e| format!("{e}"))?,
        args.second
    );
    Ok(())
}

#[cfg(feature = "test_bin_tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!(
        "FIRST: {}; SECOND: {:?}",
        args.first.contents_async().await?,
        args.second
    );
}
