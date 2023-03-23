use crate::app_config;
use crate::constants;
use crate::os;
use crate::templates;

use anyhow::{Ok, Result};

pub fn run_create_app_commands(app_name: &str, toolchain: String) -> Result<()> {
    let commands = get_create_app_commands(app_name, toolchain);
    os::run_commands(commands)?;
    Ok(())
}

fn get_create_app_commands(app_name: &str, toolchain: String) -> String {
    let toolchain_command = &templates::get_toolchain_template(&toolchain);
    let app = templates::fill_template(toolchain_command, app_name);
    let cd = templates::fill_template("cd ./NAME", app_name);
    let npm_install = get_install_command(&toolchain);
    let install_dependencies = app_config::get_app_install_deps_command(&toolchain);
    [app, cd, npm_install, install_dependencies].join("\n")
}

fn get_install_command(toolchain: &str) -> String {
    match toolchain {
        "vite" => constants::dependencies::NPM_I.to_string(),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_install_command_vite() {
        let command = get_install_command("vite");
        let expected = "npm i".to_string();
        assert_eq!(command, expected);
    }

    #[test]
    fn test_get_install_command_empty_string() {
        let command = get_install_command("");
        let expected = "".to_string();
        assert_eq!(command, expected);
    }

    #[test]
    fn test_get_install_command_default() {
        let command = get_install_command("test");
        let expected = "".to_string();
        assert_eq!(command, expected);
    }
}
