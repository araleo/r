use anyhow::Result;
use clap::Parser;
use r::constants;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(help = constants::COMMANDS_HELPER)]
    command: String,

    #[arg(short, long, help = "App/component/hook name")]
    name: Option<String>,

    #[arg(short, long, help = "Component/hook directory")]
    dir: Option<String>,

    #[arg(short, long, help = "Root directory. Defaults to components or hooks")]
    root: Option<String>,

    #[arg(short, long, help = "Toolchain to create a new app")]
    toolchain: Option<String>,

    #[arg(short, long, help = "Skip test file")]
    skip_test: bool,

    #[arg(short, long, help = "Don't create component/hook in it's own folder")]
    flat: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let name = args.name.unwrap_or("".to_string());

    let commands_with_name = vec!["ca", "nc", "nh"];
    if name.is_empty() && commands_with_name.contains(&args.command.as_str()) {
        eprintln!(
            "ERROR: {} command needs a name. Please provide it with the --name or -n option",
            args.command
        );
        std::process::exit(1);
    }

    match args.command.as_str() {
        "ca" => r::create_app(&name, args.toolchain.unwrap_or("".to_string())),
        "nc" => r::create_component(&name, args.dir, args.root, !args.skip_test, args.flat),
        "nh" => r::create_hook(&name, args.dir, args.root, !args.skip_test, args.flat),
        "lc" => r::add_lint_and_code(),
        "eslint" => r::add_eslint(),
        "vscode" => r::add_vscode(),
        _ => {
            eprintln!("ERROR: \n{}", constants::COMMANDS_HELPER);
            Ok(())
        }
    }?;

    Ok(())
}
