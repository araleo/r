mod templates;
mod utils;

use anyhow::{Ok, Result};
use std::path::Path;

pub fn create_component(
    name: &str,
    dir: Option<String>,
    root: Option<String>,
    test: bool,
    flat: bool,
    style: bool,
) -> Result<()> {
    let root_name = root.unwrap_or("components".to_string());
    let root_path = &utils::get_base_path(".", &root_name, dir)?;
    let extension = utils::get_file_extension("component");
    let path = utils::get_path_to_write(root_path, name, extension, flat);
    let template = templates::fill_template(templates::COMPONENT, name);
    utils::write_file(&path, template)?;
    if test {
        create_component_test(root_path, name, flat)?;
    }
    if style {
        create_style_file(root_path, name, flat)?;
    }
    Ok(())
}

fn create_component_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = utils::get_file_extension("comp_test");
    let test_path = utils::get_path_to_write(root, name, test_extension, flat);
    let test_template = templates::fill_template(templates::TEST, name);
    utils::write_file(&test_path, test_template)?;
    Ok(())
}

fn create_style_file(root: &Path, name: &str, flat: bool) -> Result<()> {
    let extension = utils::get_file_extension("style");
    let filepath = utils::get_path_to_write(root, name, extension, flat);
    utils::write_file(&filepath, "".to_string())?;
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
    let filename = &utils::camel_case_to_kebab_case(&mut name.to_string());
    let path = utils::get_path_to_write(root_path, filename, extension, flat);
    let template = templates::fill_template(templates::HOOK, name);
    utils::write_file(&path, template)?;
    if test {
        create_hook_test(root_path, filename, flat)?;
    }
    Ok(())
}

fn create_hook_test(root: &Path, name: &str, flat: bool) -> Result<()> {
    let test_extension = utils::get_file_extension("test");
    let test_path = utils::get_path_to_write(root, name, test_extension, flat);
    let test_template = templates::fill_template(templates::HOOK_TEST, name);
    utils::write_file(&test_path, test_template)?;
    Ok(())
}
