static TEMPLATE: &str = "const NAME = () => {};\n\nexport default NAME;";

use std::{fs::File, io::Write, path::Path};

use anyhow::Result;

pub fn create_component(component_name: &String) -> Result<()> {
    let component_path = get_component_path(component_name);
    let path = Path::new(&component_path);

    let mut file = File::create(&path)?;
    let content = TEMPLATE.replace("NAME", component_name);
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn get_component_path(component_name: &String) -> String {
    let component_path = component_name.to_owned() + ".tsx";
    component_path
}

pub fn show_available() -> Result<()> {
    println!("Available commands: rnfc");
    Ok(())
}
