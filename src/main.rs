use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    command: String,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    skip_test: bool,

    #[arg(short, long)]
    dir: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.command.as_str() {
        "nfc" => r::create_component(&args.name, &args.dir, !args.skip_test),
        "ntf" => r::create_test_file(&args.name, &args.dir),
        _ => Ok(()),
    }?;

    Ok(())
}
