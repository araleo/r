use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    command: String,
    name: String,
    dir: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command.as_str() {
        "rnfc" => r::create_component(&args.name, &args.dir),
        _ => r::show_available(),
    }?;

    println!("{:?}", args);

    Ok(())
}
