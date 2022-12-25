static TEMPLATE: &str = "const NAME = () => {};\n\nexport default NAME;\n";

use std::{fs::File, io::Write, path::Path};

use anyhow::Result;

pub fn create_component(component_name: &String, dir: &Option<String>) -> Result<()> {
    let dir_name = get_dir_name(dir);
    mkdirs(&dir_name)?;

    let component_path = get_component_path(component_name, &dir_name);
    let component_content = get_component_content(component_name);

    write_component_file(component_path, component_content)?;

    Ok(())
}

fn get_dir_name(dir: &Option<String>) -> String {
    let dir = match dir.as_deref() {
        Some(p) => p,
        None => ".",
    };
    dir.to_string()
}

fn mkdirs(dir: &String) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    Ok(())
}

fn get_component_path(name: &String, dir: &String) -> String {
    let component_path = dir.to_owned() + "/" + name + ".tsx";
    component_path
}

fn get_component_content(component_name: &String) -> String {
    let content = TEMPLATE.replace("NAME", component_name);
    content
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
