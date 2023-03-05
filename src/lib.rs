pub mod constants;
mod os;
mod templates;

use anyhow::{anyhow, Ok, Result};
use colored::Colorize;
use std::{
    ffi::OsStr,
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

pub fn create_app(name: &str, toolchain: String) -> Result<()> {
    println!("{}", "Setting up a new React App.".bold().yellow());
    run_create_app_commands(name, toolchain)?;
    println!("{}", "Adding configuration files.".bold().yellow());
    create_app_config_files(name)?;
    println!("{}", "\n\nR is done! Happy coding!\n\n".bold().green());
    Ok(())
}

fn run_create_app_commands(app_name: &str, toolchain: String) -> Result<()> {
    let commands = get_create_app_commands(app_name, toolchain);
    os::run_commands(commands)?;
    Ok(())
}

fn get_create_app_commands(app_name: &str, toolchain: String) -> String {
    let toolchain_command = &get_toolchain_template(&toolchain);
    let app = fill_template(toolchain_command, app_name);
    let cd = fill_template("cd ./NAME", app_name);
    let npm_install = get_app_install_command(toolchain);
    let install_dependencies = get_app_install_deps_command();
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

fn get_app_install_deps_command() -> String {
    let npm_install = constants::dependencies::NPM_I_DEV.to_string();
    let dependencies = constants::dependencies::DEPENDENCIES.join(" ");
    npm_install + " " + &dependencies
}

fn create_app_config_files(app_name: &str) -> Result<()> {
    let app_folder = &PathBuf::from(format!("./{app_name}")).canonicalize()?;
    os::create_prettier_file(app_folder)?;
    os::create_eslint_file(app_folder)?;
    os::create_eslint_ignore_file(app_folder)?;
    create_vscode_settings_and_snippets(app_folder)?;
    Ok(())
}

fn create_vscode_settings_and_snippets(app_folder: &Path) -> Result<()> {
    let vscode_folder = app_folder.join(".vscode");
    create_dir_all(vscode_folder.clone())?;
    os::create_vscode_settings(&vscode_folder)?;
    os::create_vscode_snippets(&vscode_folder)?;
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
    let root_path = &get_base_path(".", &root_name, dir)?;
    let extension = get_file_extension("component");
    let path = get_path_to_write(root_path, name, extension, flat);
    let template = fill_template(templates::COMPONENT, name);
    os::write_file(&path, template)?;
    if test {
        create_component_test(root_path, name, flat)?;
    }
    Ok(())
}

fn create_component_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = get_file_extension("comp_test");
    let test_path = get_path_to_write(root, name, test_extension, flat);
    let test_template = fill_template(templates::TEST, name);
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
    let root_path = &get_base_path(".", &root_name, dir)?;
    let extension = get_file_extension("hook");
    let path = get_path_to_write(root_path, name, extension, flat);
    let template = fill_template(templates::HOOK, name);
    os::write_file(&path, template)?;
    if test {
        create_hook_test(root_path, name, flat)?;
    }
    Ok(())
}

fn create_hook_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = get_file_extension("test");
    let test_path = get_path_to_write(root, name, test_extension, flat);
    let test_template = fill_template(templates::HOOK_TEST, name);
    os::write_file(&test_path, test_template)?;
    Ok(())
}

pub fn add_lint_and_code() -> Result<()> {
    add_eslint()?;
    add_vscode()?;
    Ok(())
}

pub fn add_eslint() -> Result<()> {
    let install_deps_command = get_app_install_deps_command();
    os::run_commands(install_deps_command)?;
    let app_folder = &PathBuf::from(".").canonicalize()?;
    os::create_prettier_file(app_folder)?;
    os::create_eslint_file(app_folder)?;
    os::create_eslint_ignore_file(app_folder)?;
    Ok(())
}

pub fn add_vscode() -> Result<()> {
    let app_folder = &PathBuf::from(".").canonicalize()?;
    create_vscode_settings_and_snippets(app_folder)?;
    Ok(())
}

fn get_base_path(root: &str, pattern: &str, base: Option<String>) -> Result<PathBuf> {
    let path = find_folder_by_pattern(PathBuf::from(root), pattern)?;
    Ok(path.join(base.unwrap_or("".to_string())))
}

fn find_folder_by_pattern(root: PathBuf, pattern: &str) -> Result<PathBuf> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_ignored(e))
        .filter_map(|e| e.ok());
    for entry in walker {
        if let Some(filename) = entry.path().file_name() {
            if filename == OsStr::new(pattern) {
                return Ok(entry.path().canonicalize()?);
            }
        }
    }
    let error_message = format!("ERROR: Could not find the {pattern} folder");
    Err(anyhow!(error_message))
}

fn is_ignored(entry: &DirEntry) -> bool {
    let ignore = vec!["node_modules", "dist", "__tests__", "tests"];
    entry
        .file_name()
        .to_str()
        .map(|s| ignore.contains(&s))
        .unwrap_or(false)
}

fn get_path_to_write(root: &Path, name: &str, extension: &str, flat: bool) -> PathBuf {
    let folder = if flat { "" } else { name };
    root.join(folder).join(name.to_owned() + extension)
}

fn get_file_extension(mode: &str) -> &str {
    match mode {
        "component" => ".tsx",
        "comp_test" => ".test.tsx",
        "test" => ".test.ts",
        _ => ".ts",
    }
}

fn fill_template(template: &str, component_name: &str) -> String {
    template.replace("NAME", component_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_to_write_flat() {
        let root = PathBuf::from(".");
        let expected = PathBuf::from("./test.tsx");
        assert_eq!(get_path_to_write(&root, "test", ".tsx", true), expected)
    }

    #[test]
    fn test_get_path_to_write_not_flat() {
        let root = PathBuf::from(".");
        let expected = PathBuf::from("./test/test.tsx");
        assert_eq!(get_path_to_write(&root, "test", ".tsx", false), expected)
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("component"), ".tsx");
        assert_eq!(get_file_extension("comp_test"), ".test.tsx");
        assert_eq!(get_file_extension("test"), ".test.ts");
        assert_eq!(get_file_extension("anything"), ".ts");
    }

    #[test]
    fn test_fill_template() {
        let template = "const NAME = test";
        let name = "test";
        let content = fill_template(template, name);
        let expected = "const test = test".to_string();
        assert_eq!(content, expected);
    }

    #[test]
    fn test_fill_empty_template() {
        let template = "";
        let name = "test";
        let content = fill_template(template, name);
        let expected = "".to_string();
        assert_eq!(content, expected);
    }

    #[test]
    fn test_fill_template_no_replace_match() {
        let template = "const COLOR = blue";
        let name = "test";
        let content = fill_template(template, name);
        let expected = "const COLOR = blue";
        assert_eq!(content, expected);
    }
}
