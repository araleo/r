use crate::constants;
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
    create_dir_all(vscode_folder.clone())?;
    os::create_vscode_settings(&vscode_folder)?;
    os::create_vscode_snippets(&vscode_folder)?;
    Ok(())
}

pub fn get_app_install_deps_command() -> String {
    let npm_install = constants::dependencies::NPM_I_DEV.to_string();
    let dependencies = constants::dependencies::DEPENDENCIES.join(" ");
    npm_install + " " + &dependencies
}
