use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Ok, Result};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use crate::constants::{eslint, IGNORED_DIRS};

pub fn _get_dependencies_from_eslint_file() -> Result<()> {
    // to install the dependencies I should create a map
    // where keys are the plugin name and values the plugin
    // full name (to install via npm or yarn)
    // if while parsing the file a plugin is found which is
    // not in this map we should ask for the user input
    // and store the name of the the plugin in the .r folder
    let content = eslint::get_config();
    let extends_index = content.find("extends").unwrap();
    let slice = &content[extends_index..];
    let start_bracket = slice.find('[').unwrap();
    let end_bracket = slice.find(']').unwrap();
    let extends = &slice[start_bracket..end_bracket + 1];
    let re = Regex::new(r"[\s\[\],]").unwrap();
    let extends = re.replace_all(extends, "");
    let lines: Vec<&str> = extends.split('\'').filter(|s| !s.is_empty()).collect();
    println!("{lines:?}");
    Ok(())
}

pub fn camel_case_to_kebab_case(s: &mut String) -> String {
    let uppercase_indexes = find_uppercase_indexes(s);
    for i in uppercase_indexes {
        s.insert(i, '-');
    }
    s.to_lowercase()
}

fn find_uppercase_indexes(s: &str) -> Vec<usize> {
    s.match_indices(char::is_uppercase)
        .enumerate()
        .map(|(i, (index, _))| index + i)
        .collect::<Vec<_>>()
}

pub fn get_base_path(root: &str, pattern: &str, base: Option<String>) -> Result<PathBuf> {
    let path = find_folder_by_pattern(PathBuf::from(root), pattern)?;
    Ok(path.join(base.unwrap_or("".to_string())))
}

pub fn find_folder_by_pattern(root: PathBuf, pattern: &str) -> Result<PathBuf> {
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

pub fn is_ignored(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| IGNORED_DIRS.contains(&s))
        .unwrap_or(false)
}

pub fn get_path_to_write(root: &Path, name: &str, extension: &str, flat: bool) -> PathBuf {
    let folder = if flat { "" } else { name };
    root.join(folder).join(name.to_owned() + extension)
}

pub fn get_file_extension(mode: &str) -> &str {
    match mode {
        "component" => ".tsx",
        "comp_test" => ".test.tsx",
        "test" => ".test.ts",
        "style" => ".style.tsx",
        _ => ".ts",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case_to_kebab_case() {
        let result = camel_case_to_kebab_case(&mut "useTestMeString".to_string());
        let expected = "use-test-me-string";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_camel_case_to_kebab_case_no_diff() {
        let result = camel_case_to_kebab_case(&mut "test-me".to_string());
        let expected = "test-me";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_camel_case_to_kebab_case_empty_string() {
        let result = camel_case_to_kebab_case(&mut "".to_string());
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_uppercase_indexes() {
        let indexes = find_uppercase_indexes("testTestTesterTest");
        let expected: [usize; 3] = [4, 9, 16];
        assert_eq!(indexes, expected);
    }

    #[test]
    fn test_find_uppercase_indexes_empty_string() {
        let indexes = find_uppercase_indexes("");
        let expected: [usize; 0] = [];
        assert_eq!(indexes, expected);
    }

    #[test]
    fn test_find_uppercase_indexes_lowercase_string() {
        let indexes = find_uppercase_indexes("test-test-test");
        let expected: [usize; 0] = [];
        assert_eq!(indexes, expected);
    }

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
}
