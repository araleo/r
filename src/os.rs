use anyhow::{anyhow, Ok, Result};
use std::{
    env,
    io::Write,
    process::{Child, Command, Stdio},
};

pub fn get_os_cli() -> String {
    let target_os = env::consts::OS;
    if target_os == "windows" {
        return "powershell".to_string();
    }
    "sh".to_string()
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
