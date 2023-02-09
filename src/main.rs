use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    command: String,

    #[arg(short, long, help = "Name")]
    name: String,

    #[arg(short, long, help = "Folder")]
    dir: Option<String>,

    #[arg(short, long, help = "Skip test file")]
    skip_test: bool,

    #[arg(short, long, help = "Don't create component/hook in it's own folder")]
    flat: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command.as_str() {
        "nc" => r::create_component(&args.name, args.dir, !args.skip_test, args.flat),
        "nh" => r::create_hook(&args.name, args.dir, !args.skip_test, args.flat),
        _ => Ok(()),
    }?;

    Ok(())
}
