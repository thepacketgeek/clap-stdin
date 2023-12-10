use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    first: String,
    #[clap(short, long)]
    second: Option<FileOrStdin<u32>>,
}

fn main() {
    let args = Args::parse();
    println!(
        "FIRST: {}, SECOND: {:?}",
        args.first,
        args.second.map(|second| second.contents().unwrap()),
    );
}
