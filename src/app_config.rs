use crate::constants::dependencies;
use crate::os;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::{Ok, Result};

pub fn create_app_config_files(app_name: &str) -> Result<()> {
    let app_folder = &PathBuf::from(format!("./{app_name}")).canonicalize()?;
    os::create_prettier_file(app_folder)?;
    os::create_eslint_file(app_folder)?;
    os::create_eslint_ignore_file(app_folder)?;
    create_vscode_settings_and_snippets(app_folder)?;
    Ok(())
}

pub fn create_vscode_settings_and_snippets(app_folder: &Path) -> Result<()> {
    let vscode_folder = app_folder.join(".vscode");
    create_dir_all(&vscode_folder)?;
    os::create_vscode_settings(&vscode_folder)?;
    os::create_vscode_snippets(&vscode_folder)?;
    Ok(())
}

pub fn get_app_install_deps_command(toolchain: &str) -> String {
    let npm_i_dev = dependencies::NPM_I_DEV.to_string();
    let lint_deps = dependencies::LINT_DEPENDENCIES.join(" ");
    let toolchain_deps = get_toolchain_dependencies(toolchain);
    npm_i_dev + " " + &lint_deps + " " + &toolchain_deps
}

pub fn get_install_lint_dependencies_command() -> String {
    let npm_i_dev = dependencies::NPM_I_DEV.to_string();
    let deps = dependencies::LINT_DEPENDENCIES.join(" ");
    npm_i_dev + " " + &deps
}

pub fn get_toolchain_dependencies(toolchain: &str) -> String {
    match toolchain {
        "vite" => dependencies::VITE_DEPENDENCIES.join(" "),
        "cra" => dependencies::CRA_DEPENDENCIES.join(" "),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_install_lint_dependencies_command() {
        let command = get_install_lint_dependencies_command();
        let expected = "npm i -D ".to_string() + &dependencies::LINT_DEPENDENCIES.join(" ");
        assert_eq!(command, expected);
    }

    #[test]
    fn test_get_toolchain_dependencies_vite() {
        let command = get_toolchain_dependencies("vite");
        let expected = dependencies::VITE_DEPENDENCIES.join(" ");
        assert_eq!(command, expected);
    }

    #[test]
    fn test_get_toolchain_dependencies_cra() {
        let command = get_toolchain_dependencies("cra");
        let expected = dependencies::CRA_DEPENDENCIES.join(" ");
        assert_eq!(command, expected);
    }

    #[test]
    fn test_get_toolchain_dependencies_default() {
        let command = get_toolchain_dependencies("test");
        assert_eq!(command, "".to_string());
    }
}
