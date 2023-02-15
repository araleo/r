use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(help = "Available commands: ca, nc, nh")]
    command: String,

    #[arg(short, long, help = "Name")]
    name: String,

    #[arg(short, long, help = "Folder")]
    dir: Option<String>,

    #[arg(short, long, help = "Skip test file")]
    skip_test: bool,

    #[arg(short, long, help = "Don't create component/hook in it's own folder")]
    flat: bool,

    #[arg(short, long, help = "Toolchain to create a new app")]
    toolchain: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command.as_str() {
        "ca" => r::create_app(&args.name, args.toolchain.unwrap_or("vite".to_string())),
        "nc" => r::create_component(&args.name, args.dir, !args.skip_test, args.flat),
        "nh" => r::create_hook(&args.name, args.dir, !args.skip_test, args.flat),
        _ => {
            eprintln!("ERROR: Available commands:\nna: New app\nnc: New component\nnh: New hook");
            Ok(())
        }
    }?;

    Ok(())
}
