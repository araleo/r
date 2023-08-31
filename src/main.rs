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

    #[arg(short, long, help = "Skip test file")]
    skip_test: bool,

    #[arg(short, long, help = "Don't create component/hook in it's own folder")]
    flat: bool,

    #[arg(long, help = "Create .style.tsx file when creating component")]
    style: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let command = args.command.as_str();
    let name = args.name.unwrap_or("".to_string());
    check_name(&name, command)?;

    match command {
        "nc" => r::create_component(
            &name,
            args.dir,
            args.root,
            !args.skip_test,
            args.flat,
            args.style,
        ),
        "nh" => r::create_hook(&name, args.dir, args.root, !args.skip_test, args.flat),
        _ => command_error(),
    }?;

    Ok(())
}

fn check_name(name: &String, command: &str) -> Result<()> {
    if name.is_empty() {
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

const COMMANDS_HELPER: &str = "Available commands:
na: New App (new app)
nc: New component (new component)
nh: New hook (new hook)
lc: Adds eslint and vscode settings and snippets to an existing app (lint and code)
eslint: Adds eslint to an existing app (eslint)
vscode: Adds vscode settings and snippets to an existing app (vscode)
";
