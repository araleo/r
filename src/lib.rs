mod os;
mod templates;

use anyhow::{anyhow, Ok, Result};
use colored::Colorize;
use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

// TODO: run npm install eslint and its plugins
// TODO: update eslint template to a reasonable one
pub fn create_app(name: &str) -> Result<()> {
    println!("{}", "Setting up a new Vite React App.".bold().yellow());
    run_create_app_commands(name)?;
    println!("{}", "Adding configuration files.".bold().yellow());
    create_app_config_files(name)?;
    println!("{}", "Done! Happy coding!".bold().green());
    Ok(())
}

fn run_create_app_commands(app_name: &str) -> Result<()> {
    let target_cli = os::get_os_cli();
    let mut child_process = os::get_child_process(&target_cli);
    let commands = get_create_app_commands(app_name);
    os::run_command_on_child(&mut child_process, commands.as_bytes())?;
    child_process.wait_with_output()?;
    Ok(())
}

fn get_create_app_commands(app_name: &str) -> String {
    let vite_command = fill_template(templates::VITE, app_name);
    let cd_command = fill_template("cd ./NAME", app_name);
    let npm_install = "npm install".to_string();
    [vite_command, cd_command, npm_install].join("\n")
}

fn create_app_config_files(app_name: &str) -> Result<()> {
    let app_folder = &PathBuf::from(format!("./{app_name}")).canonicalize()?;
    create_prettier_file(app_folder)?;
    create_eslint_file(app_folder)?;
    Ok(())
}

fn create_prettier_file(app_folder: &Path) -> Result<()> {
    let filepath = app_folder.join(".prettierrc.json");
    write_file(&filepath, templates::PRETTIER.to_string())?;
    Ok(())
}

fn create_eslint_file(app_folder: &Path) -> Result<()> {
    let filepath = app_folder.join(".eslintrc.js");
    write_file(&filepath, templates::ESLINT.to_string())?;
    Ok(())
}

pub fn create_component(name: &str, dir: Option<String>, test: bool, flat: bool) -> Result<()> {
    let root = get_base_path(".", "components", dir)?;
    let extension = get_file_extension("component");
    let path = get_path_to_write(&root, name, extension, flat);
    let template = fill_template(templates::COMPONENT, name);
    write_file(&path, template)?;
    if test {
        create_component_test(&root, name, flat)?;
    }
    Ok(())
}

fn create_component_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = get_file_extension("comp_test");
    let test_path = get_path_to_write(root, name, test_extension, flat);
    let test_template = fill_template(templates::TEST, name);
    write_file(&test_path, test_template)?;
    Ok(())
}

pub fn create_hook(name: &str, dir: Option<String>, test: bool, flat: bool) -> Result<()> {
    let root = get_base_path(".", "hooks", dir)?;
    let extension = get_file_extension("hook");
    let path = get_path_to_write(&root, name, extension, flat);
    let template = fill_template(templates::HOOK, name);
    write_file(&path, template)?;
    if test {
        create_hook_test(&root, name, flat)?;
    }
    Ok(())
}

fn create_hook_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = get_file_extension("test");
    let test_path = get_path_to_write(root, name, test_extension, flat);
    let test_template = fill_template(templates::HOOK_TEST, name);
    write_file(&test_path, test_template)?;
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
        println!("{entry:?}");
        if let Some(filename) = entry.path().file_name() {
            if filename == OsStr::new(pattern) {
                return Ok(entry.path().canonicalize()?);
            }
        }
    }
    return Err(anyhow!("Pattern not found"));
}

fn is_ignored(entry: &DirEntry) -> bool {
    let ignore = vec!["node_modules", "dist", "__tests__", "tests"];
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') || ignore.contains(&s))
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

fn write_file(filepath: &PathBuf, content: String) -> Result<()> {
    let parent = filepath.parent().unwrap();
    create_dir_all(parent)?;
    let mut file = File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
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
