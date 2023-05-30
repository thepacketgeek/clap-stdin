use clap::Parser;

use clap_stdin::MaybeStdin;

#[derive(Debug, Parser)]
struct Args {
    first: MaybeStdin<String>,
    second: MaybeStdin<u32>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
