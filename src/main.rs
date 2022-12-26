use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    command: String,
    name: String,
    dir: Option<String>,
    #[arg(short, long, default_value_t = true)]
    test: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command.as_str() {
        "rnfc" => r::create_component(&args.name, &args.dir, args.test),
        "rntf" => r::create_test_file(&args.name),
        _ => r::show_available(),
    }?;

    println!("{:?}", args);

    Ok(())
}
