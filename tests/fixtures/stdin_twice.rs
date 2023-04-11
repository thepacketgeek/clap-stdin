use clap::Parser;

use clap_stdin::MaybeStdIn;

#[derive(Debug, Parser)]
struct Args {
    first: MaybeStdIn<String>,
    second: MaybeStdIn<u32>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
