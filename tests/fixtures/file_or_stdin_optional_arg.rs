use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    first: String,
    #[arg(short, long)]
    second: Option<FileOrStdin<u32>>,
}

#[cfg(feature = "test_bin")]
fn main() {
    let args = Args::parse();
    println!(
        "FIRST: {}, SECOND: {:?}",
        args.first,
        args.second.map(|second| second.contents().unwrap()),
    );
}

#[cfg(feature = "test_bin_tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!(
        "FIRST: {}, SECOND: {:?}",
        args.first,
        args.second
            .map(|second| second.contents_async().await.unwrap()),
    );
}
