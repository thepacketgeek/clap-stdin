use clap::Parser;

use clap_stdin::MaybeStdin;

#[derive(Debug, Parser)]
struct Args {
    first: MaybeStdin<String>,
    #[arg(short, long)]
    second: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
