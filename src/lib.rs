static TEMPLATE: &str = "const NAME = () => {};\n\nexport default NAME;\n";

use std::{fs::File, io::Write, path::Path};

use anyhow::Result;

pub fn create_component(name: &String, dir: &Option<String>) -> Result<()> {
    let dir_name = get_dir_name(dir);
    mkdirs(&dir_name)?;

    let path = get_component_path(name, &dir_name);
    let content = get_component_content(name);

    write_component_file(path, content)?;

    Ok(())
}

fn get_dir_name(dir: &Option<String>) -> String {
    let dir = match dir.as_deref() {
        Some(p) => p,
        None => ".",
    };
    dir.to_string()
}

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

fn mkdirs(dir: &String) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    Ok(())
}

fn get_component_path(name: &String, dir: &String) -> String {
    let component_path = dir.to_owned() + "/" + name + ".tsx";
    component_path
}

#[test]
fn test_get_component_path() {
    let name = "Button".to_string();
    let dir = "UI".to_string();
    let component_path = get_component_path(&name, &dir);
    assert_eq!(component_path, "UI/Button.tsx");
}

#[test]
fn test_get_component_path_cwd() {
    let name = "Button".to_string();
    let dir = ".".to_string();
    let component_path = get_component_path(&name, &dir);
    assert_eq!(component_path, "./Button.tsx");
}

fn get_component_content(name: &String) -> String {
    let content = TEMPLATE.replace("NAME", name);
    content
}

#[test]
fn test_get_component_content() {
    let name = "Button".to_string();
    let content = get_component_content(&name);
    let expected = "const Button = () => {};\n\nexport default Button;\n".to_string();
    assert_eq!(content, expected);
}

fn write_component_file(filepath: String, content: String) -> Result<()> {
    let path = Path::new(&filepath);
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn show_available() -> Result<()> {
    println!("Available commands: rnfc");
    Ok(())
}
