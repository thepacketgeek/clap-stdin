use clap::{CommandFactory, Parser};

use clap_complete::engine::{ArgValueCompleter, PathCompleter, ValueCompleter};
use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[arg(add = ArgValueCompleter::new(|x: &std::ffi::OsStr| PathCompleter::file().stdio().complete(x)), default_value = "-")]
    value: FileOrStdin,
}

#[cfg(feature = "test_bin")]
fn main() -> Result<(), String> {
    clap_complete::CompleteEnv::with_factory(Args::command).complete();

    let args = Args::parse();
    println!(
        "VALUE: {}",
        args.value.contents().map_err(|e| format!("{e}"))?
    );
    Ok(())
}

#[cfg(feature = "test_bin_tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    clap_complete::CompleteEnv::with_factory(Args::command).complete();

    let args = Args::parse();
    println!("VALUE: {}", args.value.contents_async().await?);
}
