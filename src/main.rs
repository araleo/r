use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    command: String,

    #[arg(short, long, help = "Component name")]
    name: String,

    #[arg(short, long, help = "Component folder")]
    dir: Option<String>,

    #[arg(short, long, help = "Skip creating test file")]
    skip_test: bool,

    #[arg(short, long, help = "Creates the component in a subfolder")]
    folder: bool,

    #[arg(short, long, help = "Uses the components folder as base dir")]
    component: bool,

    #[arg(short, long, help = "Uses the pages folder as base dir")]
    page: bool,

    #[arg(
        short,
        long,
        help = "Uses the hooks folder as base dir and creates a base test for a react hook"
    )]
    hook: bool,

    #[arg(short, long, help = "Folder to write test files")]
    test_folder: Option<String>,

    #[arg(short, long, help = "Folders and paths to ignore")]
    ignore: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command.as_str() {
        "nfc" => r::create_component(
            &args.name,
            &args.dir,
            args.component,
            args.folder,
            !args.skip_test,
            args.test_folder,
        ),
        "ntf" => r::create_test_file(&args.name, &args.dir),
        "nh" => r::create_hook(),
        _ => Ok(()),
    }?;

    Ok(())
}
