use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    first: FileOrStdin,
    #[clap(short, long)]
    second: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
