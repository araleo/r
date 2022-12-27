mod templates;

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::{Ok, Result};

pub fn create_component(
    component_name: &String,
    dir: &Option<String>,
    find_components_folder: bool,
    create_component_folder: bool,
    test: bool,
) -> Result<()> {
    let component_dir = get_base_dir(
        dir,
        find_components_folder,
        create_component_folder,
        component_name,
    )?;
    std::fs::create_dir_all(&component_dir)?;

    let component_path = get_component_filepath(component_name, &component_dir);
    let component_content = fill_template(templates::COMPONENT.to_string(), component_name);
    write_file(&component_path, component_content)?;

    if test {
        let test_path = get_test_filepath(component_name, &component_dir);
        let test_content = fill_template(templates::TEST.to_string(), component_name);
        write_file(&test_path, test_content)?;
    }

    Ok(())
}

fn get_base_dir(
    dir: &Option<String>,
    find_components_folder: bool,
    create_component_folder: bool,
    component_name: &String,
) -> Result<String> {
    if !find_components_folder {
        return Ok(dir.as_deref().unwrap_or(".").to_string());
    }

    let components_dir = get_components_folder_dir(None)?;
    let component_folder = dir.as_deref().unwrap_or("");
    let folder_name = get_component_folder_name(create_component_folder, component_name);
    if component_folder.is_empty() {
        Ok(components_dir + &folder_name)
    } else {
        Ok(components_dir + "/" + component_folder + &folder_name)
    }
}

fn get_component_folder_name(create_folder: bool, component_name: &String) -> String {
    if create_folder {
        "/".to_string() + component_name
    } else {
        "".to_string()
    }
}

pub fn get_components_folder_dir(dir: Option<String>) -> Result<String> {
    let base_dir = dir.unwrap_or_else(get_cwd_string);
    let path = Path::new(&base_dir);
    if path.file_name().unwrap().to_string_lossy() == "components" {
        return Ok(path.to_string_lossy().to_string());
    }
    walk_dir(base_dir)
}

fn get_cwd_string() -> String {
    env::current_dir()
        .as_deref()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

fn walk_dir(dir: String) -> Result<String> {
    let ignore = vec![".git", "node_modules", "dist", "__tests__", "tests"];

    let path = Path::new(&dir);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_name = entry.file_name().to_string_lossy().to_string();
        let entry_path = entry.path();
        let entry_path_string = entry_path.to_string_lossy().to_string();

        if entry_name == "components" {
            return Ok(entry_path_string);
        }

        if entry_path.is_dir() && !ignore.contains(&entry_name.as_str()) {
            let ok = walk_dir(entry_path_string)?;
            if !ok.is_empty() {
                return Ok(ok);
            }
        }
    }

    Ok("".to_string())
}

fn get_component_filepath(component_name: &String, component_dir: &String) -> String {
    component_dir.to_owned() + "/" + component_name + ".tsx"
}

fn get_test_filepath(component_name: &String, test_dir: &String) -> String {
    test_dir.to_owned() + "/" + component_name + ".test.tsx"
}

fn fill_template(template: String, component_name: &str) -> String {
    template.replace("NAME", component_name)
}

fn write_file(filepath: &String, content: String) -> Result<()> {
    let path = Path::new(filepath);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn create_test_file(component_name: &String, dir: &Option<String>) -> Result<()> {
    let dir_name = get_base_dir(dir, false, false, component_name)?;
    std::fs::create_dir_all(&dir_name)?;

    let content = fill_template(templates::TEST.to_string(), component_name);
    let filepath = get_test_filepath(component_name, &dir_name);
    write_file(&filepath, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dir_name() -> Result<()> {
        let dir = Some("UI".to_string());
        let dir_name = get_base_dir(&dir, false, false, &"".to_string())?;
        assert_eq!(dir_name, "UI");
        Ok(())
    }

    #[test]
    fn test_get_nested_dir_name() -> Result<()> {
        let dir = Some("UI/Buttons/Button".to_string());
        let dir_name = get_base_dir(&dir, false, false, &"".to_string())?;
        assert_eq!(dir_name, "UI/Buttons/Button");
        Ok(())
    }

    #[test]
    fn test_get_cwd_name() -> Result<()> {
        let dir = None;
        let dir_name = get_base_dir(&dir, false, false, &"".to_string())?;
        assert_eq!(dir_name, ".");
        Ok(())
    }

    #[test]
    fn test_get_component_filepath() {
        let name = "Button".to_string();
        let dir = "UI".to_string();
        let component_path = get_component_filepath(&name, &dir);
        assert_eq!(component_path, "UI/Button.tsx");
    }

    #[test]
    fn test_get_component_nested_filepath() {
        let name = "Button".to_string();
        let dir = "UI/Buttons/Test".to_string();
        let component_path = get_component_filepath(&name, &dir);
        assert_eq!(component_path, "UI/Buttons/Test/Button.tsx");
    }

    #[test]
    fn test_get_component_filepath_cwd() {
        let name = "Button".to_string();
        let dir = ".".to_string();
        let component_path = get_component_filepath(&name, &dir);
        assert_eq!(component_path, "./Button.tsx");
    }

    #[test]
    fn test_get_test_filepath() {
        let name = "Button".to_string();
        let dir = "UI".to_string();
        let test_path = get_test_filepath(&name, &dir);
        assert_eq!(test_path, "UI/Button.test.tsx");
    }

    #[test]
    fn test_get_test_nested_filepath() {
        let name = "Button".to_string();
        let dir = "UI/Buttons/Test".to_string();
        let test_path = get_test_filepath(&name, &dir);
        assert_eq!(test_path, "UI/Buttons/Test/Button.test.tsx");
    }

    #[test]
    fn test_get_test_filepath_cwd() {
        let name = "Button".to_string();
        let dir = ".".to_string();
        let test_path = get_test_filepath(&name, &dir);
        assert_eq!(test_path, "./Button.test.tsx");
    }

    #[test]
    fn test_fill_template() {
        let template = "const NAME = test".to_string();
        let name = "test".to_string();
        let content = fill_template(template, &name);
        let expected = "const test = test".to_string();
        assert_eq!(content, expected);
    }

    #[test]
    fn test_fill_empty_template() {
        let template = "".to_string();
        let name = "test".to_string();
        let content = fill_template(template, &name);
        let expected = "".to_string();
        assert_eq!(content, expected);
    }

    #[test]
    fn test_fill_template_no_replace_match() {
        let template = "const COLOR = blue".to_string();
        let name = "test".to_string();
        let content = fill_template(template, &name);
        let expected = "const COLOR = blue".to_string();
        assert_eq!(content, expected);
    }
}
