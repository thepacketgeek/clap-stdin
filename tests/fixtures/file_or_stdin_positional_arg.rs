use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    #[clap(default_value = "-")]
    first: FileOrStdin,
    #[clap(short, long)]
    second: Option<String>,
}

#[cfg(feature = "test_bin")]
fn main() {
    let args = Args::parse();
    println!(
        "FIRST: {}; SECOND: {:?}",
        args.first.contents().unwrap(),
        args.second
    );
}

#[cfg(feature = "test_bin_tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!(
        "FIRST: {}; SECOND: {:?}",
        args.first.contents_async().await.unwrap(),
        args.second
    );
}
