mod app_config;
pub mod constants;
mod new_app;
mod os;
mod templates;
mod utils;

use crate::app_config::create_app_config_files;
use anyhow::{Ok, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn create_app(name: &str, toolchain: String) -> Result<()> {
    println!("{}", "Setting up a new React App.".bold().yellow());
    new_app::run_create_app_commands(name, toolchain)?;
    println!("{}", "Adding configuration files.".bold().yellow());
    create_app_config_files(name)?;
    println!("{}", "\n\nR is done! Happy coding!\n\n".bold().green());
    Ok(())
}

pub fn create_component(
    name: &str,
    dir: Option<String>,
    root: Option<String>,
    test: bool,
    flat: bool,
) -> Result<()> {
    let root_name = root.unwrap_or("components".to_string());
    let root_path = &utils::get_base_path(".", &root_name, dir)?;
    let extension = utils::get_file_extension("component");
    let path = utils::get_path_to_write(root_path, name, extension, flat);
    let template = templates::fill_template(templates::COMPONENT, name);
    os::write_file(&path, template)?;
    if test {
        create_component_test(root_path, name, flat)?;
    }
    Ok(())
}

fn create_component_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = utils::get_file_extension("comp_test");
    let test_path = utils::get_path_to_write(root, name, test_extension, flat);
    let test_template = templates::fill_template(templates::TEST, name);
    os::write_file(&test_path, test_template)?;
    Ok(())
}

pub fn create_hook(
    name: &str,
    dir: Option<String>,
    root: Option<String>,
    test: bool,
    flat: bool,
) -> Result<()> {
    let root_name = root.unwrap_or("hooks".to_string());
    let root_path = &utils::get_base_path(".", &root_name, dir)?;
    let extension = utils::get_file_extension("hook");
    let path = utils::get_path_to_write(root_path, name, extension, flat);
    let template = templates::fill_template(templates::HOOK, name);
    os::write_file(&path, template)?;
    if test {
        create_hook_test(root_path, name, flat)?;
    }
    Ok(())
}

fn create_hook_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = utils::get_file_extension("test");
    let test_path = utils::get_path_to_write(root, name, test_extension, flat);
    let test_template = templates::fill_template(templates::HOOK_TEST, name);
    os::write_file(&test_path, test_template)?;
    Ok(())
}

pub fn add_lint_and_code() -> Result<()> {
    add_eslint()?;
    add_vscode()?;
    Ok(())
}

pub fn add_eslint() -> Result<()> {
    let install_deps_command = app_config::get_app_install_deps_command();
    os::run_commands(install_deps_command)?;
    let app_folder = &PathBuf::from(".").canonicalize()?;
    os::create_prettier_file(app_folder)?;
    os::create_eslint_file(app_folder)?;
    os::create_eslint_ignore_file(app_folder)?;
    Ok(())
}

pub fn add_vscode() -> Result<()> {
    let app_folder = &PathBuf::from(".").canonicalize()?;
    app_config::create_vscode_settings_and_snippets(app_folder)?;
    Ok(())
}
