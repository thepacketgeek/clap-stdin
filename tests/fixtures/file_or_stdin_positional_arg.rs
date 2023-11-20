use clap::Parser;

use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
struct Args {
    #[clap(default_value = "-")]
    first: FileOrStdin,
    #[clap(short, long)]
    second: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!(
        "FIRST: {}; SECOND: {:?}",
        args.first.contents().unwrap(),
        args.second
    );
}
