use anyhow::{anyhow, Ok, Result};
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(help = COMMANDS_HELPER)]
    command: String,

    #[arg(short, long, help = "App/component/hook name")]
    name: Option<String>,

    #[arg(short, long, help = "Component/hook directory")]
    dir: Option<String>,

    #[arg(short, long, help = "Root directory. Defaults to components or hooks")]
    root: Option<String>,

    #[arg(short, long, help = "Don't create component/hook in it's own folder")]
    flat: bool,

    #[arg(long, help = "Create .style.tsx file when creating component")]
    style: bool,

    #[arg(long, help = "Create .stories.tsx file when creating component")]
    story: bool,

    #[arg(long, help = "Create .test.tsx file when creating component")]
    test: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let command = args.command.as_str();
    let name = args.name.unwrap_or("".to_string());
    check_name(&name, command)?;

    match command {
        "nc" => r::create_component(
            &name, args.dir, args.root, args.flat, args.test, args.style, args.story,
        ),
        "nh" => r::create_hook(&name, args.dir, args.root, args.test, args.flat),
        "version" => print_version(),
        _ => command_error(),
    }?;

    Ok(())
}

fn check_name(name: &String, command: &str) -> Result<()> {
    if name.is_empty() && command != "version" {
        return Err(anyhow!(
            "ERROR: {command} command needs a name. Please provide it with the --name or -n option"
        ));
    }
    Ok(())
}

fn command_error() -> Result<()> {
    eprintln!("ERROR: \n{}", COMMANDS_HELPER);
    Ok(())
}

fn print_version() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("R Version: {}", VERSION);
    Ok(())
}

const COMMANDS_HELPER: &str = "Available commands:
nc: New component (new component)
nh: New hook (new hook)
";
