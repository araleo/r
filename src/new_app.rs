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
    let toolchain_command = &get_toolchain_template(&toolchain);
    let app = templates::fill_template(toolchain_command, app_name);
    let cd = templates::fill_template("cd ./NAME", app_name);
    let npm_install = get_app_install_command(toolchain);
    let install_dependencies = app_config::get_app_install_deps_command();
    [app, cd, npm_install, install_dependencies].join("\n")
}

fn get_toolchain_template(toolchain: &str) -> String {
    match toolchain {
        "cra" => templates::CRA.to_string(),
        "vite" => templates::VITE.to_string(),
        _ => templates::VITE.to_string(),
    }
}

fn get_app_install_command(toolchain: String) -> String {
    if toolchain == "vite" {
        constants::dependencies::NPM_I.to_string()
    } else {
        "".to_string()
    }
}
