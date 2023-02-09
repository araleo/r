mod templates;

use anyhow::{anyhow, Ok, Result};
use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

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
