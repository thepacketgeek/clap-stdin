use clap::Parser;

use clap_stdin::MaybeStdIn;

#[derive(Debug, Parser)]
struct Args {
    first: String,
    #[clap(short, long)]
    second: Option<MaybeStdIn<u32>>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
