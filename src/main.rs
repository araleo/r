use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    command: String,
    name: String,
    path: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command.as_str() {
        "rnfc" => r::create_component(&args.name),
        _ => r::show_available(),
    }?;

    println!("{:?}", args);

    Ok(())
}
