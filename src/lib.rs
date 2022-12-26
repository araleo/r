use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use anyhow::{Ok, Result};

pub fn create_component(name: &String, dir: &Option<String>, test: bool) -> Result<()> {
    let dir_name = get_dir_name(dir);
    std::fs::create_dir_all(&dir_name)?;

    let component_path = get_component_path(name, &dir_name);
    let component_content = get_component_content(name)?;
    write_file(&component_path, component_content)?;

    if test {
        let test_path = get_test_path(&name, &dir_name);
        let test_content = get_test_content(name)?;
        write_file(&test_path, test_content)?;
    }

    Ok(())
}

pub fn create_test_file(component_name: &String, dir: &Option<String>) -> Result<()> {
    let dir_name = get_dir_name(dir);
    std::fs::create_dir_all(&dir_name)?;

    let content = get_test_content(component_name)?;
    let filepath = get_test_path(component_name, &dir_name);
    write_file(&filepath, content)?;

    Ok(())
}

fn get_dir_name(dir: &Option<String>) -> String {
    let dir = match dir.as_deref() {
        Some(p) => p,
        None => ".",
    };
    dir.to_string()
}

fn get_component_path(component_name: &String, component_dir: &String) -> String {
    let component_path = component_dir.to_owned() + "/" + component_name + ".tsx";
    component_path
}

fn get_test_path(component_name: &String, test_dir: &String) -> String {
    let test_path = test_dir.to_owned() + "/" + component_name + ".test.tsx";
    test_path
}

fn get_component_content(name: &String) -> Result<String> {
    let tmpl_path = "./templates/component.txt".to_string();
    let template = read_file(&tmpl_path)?;
    let content = fill_template(template, name);
    Ok(content)
}

fn get_test_content(component_name: &String) -> Result<String> {
    let tmpl_path = "./templates/test.txt".to_string();
    let template = read_file(&tmpl_path)?;
    let content = fill_template(template, component_name);
    Ok(content)
}

fn fill_template(template: String, component_name: &String) -> String {
    let content = template.replace("NAME", component_name);
    content
}

fn write_file(filepath: &String, content: String) -> Result<()> {
    let path = Path::new(filepath);
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn read_file(filepath: &String) -> Result<String> {
    let tmpl_path = Path::new(filepath);
    let mut file = File::open(&tmpl_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dir_name() {
        let dir = Some("UI".to_string());
        let dir_name = get_dir_name(&dir);
        assert_eq!(dir_name, "UI")
    }

    #[test]
    fn test_get_nested_dir_name() {
        let dir = Some("UI/Buttons/Button".to_string());
        let dir_name = get_dir_name(&dir);
        assert_eq!(dir_name, "UI/Buttons/Button")
    }

    #[test]
    fn test_get_cwd_name() {
        let dir = None;
        let dir_name = get_dir_name(&dir);
        assert_eq!(dir_name, ".")
    }

    #[test]
    fn test_get_component_path() {
        let name = "Button".to_string();
        let dir = "UI".to_string();
        let component_path = get_component_path(&name, &dir);
        assert_eq!(component_path, "UI/Button.tsx");
    }

    #[test]
    fn test_get_component_nested_path() {
        let name = "Button".to_string();
        let dir = "UI/Buttons/Test".to_string();
        let component_path = get_component_path(&name, &dir);
        assert_eq!(component_path, "UI/Buttons/Test/Button.tsx");
    }

    #[test]
    fn test_get_component_path_cwd() {
        let name = "Button".to_string();
        let dir = ".".to_string();
        let component_path = get_component_path(&name, &dir);
        assert_eq!(component_path, "./Button.tsx");
    }

    #[test]
    fn test_get_test_path() {
        let name = "Button".to_string();
        let dir = "UI".to_string();
        let test_path = get_test_path(&name, &dir);
        assert_eq!(test_path, "UI/Button.test.tsx");
    }

    #[test]
    fn test_get_test_nested_path() {
        let name = "Button".to_string();
        let dir = "UI/Buttons/Test".to_string();
        let test_path = get_test_path(&name, &dir);
        assert_eq!(test_path, "UI/Buttons/Test/Button.test.tsx");
    }

    #[test]
    fn test_get_test_path_cwd() {
        let name = "Button".to_string();
        let dir = ".".to_string();
        let test_path = get_test_path(&name, &dir);
        assert_eq!(test_path, "./Button.test.tsx");
    }

    #[test]
    fn test_get_component_content() -> Result<()> {
        let name = "Button".to_string();
        let file_content = get_component_content(&name)?;
        let content = file_content.replace("\r\n", "\n");
        let expected = "const Button = () => {};\n\nexport default Button;\n".to_string();
        assert_eq!(content, expected);
        Ok(())
    }

    #[test]
    fn test_get_test_content() -> Result<()> {
        let name = "Button".to_string();
        let file_content = get_test_content(&name)?;
        let content = file_content.replace("\r\n", "\n");
        let expected = "import { render, screen } from '@testing-library/react';\nimport Button from './Button';\n\ndescribe('Button tests', () => {\n  test('Button renders', () => {\n    render(<Button />);\n  });\n});\n".to_string();
        assert_eq!(content, expected);
        Ok(())
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
