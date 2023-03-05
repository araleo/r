use anyhow::{anyhow, Ok, Result};
use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

use crate::constants;

pub fn run_commands(commands: String) -> Result<()> {
    let target_cli = get_os_cli();
    let mut child_process = get_child_process(&target_cli);
    run_command_on_child(&mut child_process, commands.as_bytes())?;
    child_process.wait_with_output()?;
    Ok(())
}

pub fn get_os_cli() -> String {
    match env::consts::OS {
        "windows" => "powershell".to_string(),
        _ => "sh".to_string(),
    }
}

pub fn get_child_process(cli: &str) -> Child {
    let child = Command::new(cli).stdin(Stdio::piped()).spawn().unwrap();
    child
}

pub fn run_command_on_child(child: &mut Child, command: &[u8]) -> Result<()> {
    child
        .stdin
        .as_mut()
        .ok_or(anyhow!("Failed to capture stdin"))?
        .write_all(command)?;
    Ok(())
}

pub fn write_file(filepath: &PathBuf, content: String) -> Result<()> {
    let parent = filepath.parent().unwrap();
    create_dir_all(parent)?;
    let mut file = File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn create_prettier_file(dir: &Path) -> Result<()> {
    let filepath = dir.join(".prettierrc.json");
    write_file(&filepath, constants::prettier::get_config())?;
    Ok(())
}

pub fn create_eslint_file(dir: &Path) -> Result<()> {
    let filepath = dir.join(".eslintrc.cjs");
    write_file(&filepath, constants::eslint::get_config())?;
    Ok(())
}

pub fn create_eslint_ignore_file(dir: &Path) -> Result<()> {
    let filepath = dir.join(".eslintignore");
    write_file(&filepath, constants::eslint::get_ignore())?;
    Ok(())
}

pub fn create_vscode_settings(dir: &Path) -> Result<()> {
    let filepath = dir.join("settings.json");
    write_file(&filepath, constants::vscode::get_settings())?;
    Ok(())
}

pub fn create_vscode_snippets(dir: &Path) -> Result<()> {
    let filepath = dir.join("react.code-snippets.json");
    write_file(&filepath, constants::vscode::get_snippets())?;
    Ok(())
}
