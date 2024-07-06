use clap::Parser;

use clap_stdin::{FileOrStdin, MaybeStdin};

#[derive(Debug, Parser)]
struct Args {
    first: FileOrStdin,
    second: MaybeStdin<u32>,
}

#[cfg(feature = "test_bin")]
fn main() -> Result<(), String> {
    let args = Args::parse();
    println!(
        "FIRST is_stdin: {}; SECOND is_stdin: {}",
        args.first.is_stdin(),
        args.second.is_stdin(),
    );

    Ok(())
}

#[cfg(feature = "test_bin_tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!(
        "FIRST is_stdin: {}; SECOND is_stdin: {}",
        args.first.is_stdin(),
        args.second.is_stdin(),
    );
}
