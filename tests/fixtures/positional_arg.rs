use clap::Parser;

use clap_stdin::MaybeStdIn;

#[derive(Debug, Parser)]
struct Args {
    first: MaybeStdIn<String>,
    #[clap(short, long)]
    second: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
