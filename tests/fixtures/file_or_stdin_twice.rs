use clap::Parser;

use clap_stdin::{FileOrStdin, MaybeStdin};

#[derive(Debug, Parser)]
struct Args {
    first: FileOrStdin,
    second: MaybeStdin<u32>,
}

fn main() {
    let args = Args::parse();
    println!(
        "FIRST: {}; SECOND: {}",
        args.first.contents().unwrap(),
        args.second
    );
}
