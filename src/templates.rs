pub const COMPONENT: &str = "const NAME = () => {};

export default NAME;
";

pub const TEST: &str = "import { render, screen } from '@testing-library/react';
import NAME from './NAME';

describe('NAME tests', () => {
  test('NAME renders', () => {
    render(<NAME />);
  });
});
";

pub const HOOK: &str = "const NAME = () => {
  return { };
};

export default NAME;
";

pub const HOOK_TEST: &str = "import { act, renderHook } from '@testing-library/react';
import NAME from './NAME';

describe('NAME hook tests', () => {
  test('NAME inits ', () => {
    const { result } = renderHook(() => NAME());
  });
});
";

pub const CRA: &str = "npx create-react-app NAME --template typescript";

pub const VITE: &str = "npm create vite@latest NAME -- --template=react-ts";

pub fn fill_template(template: &str, component_name: &str) -> String {
    template.replace("NAME", component_name)
}

pub fn get_toolchain_template(toolchain: &str) -> String {
    match toolchain {
        "cra" => CRA.to_string(),
        "vite" => VITE.to_string(),
        _ => VITE.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_get_toolchain_template_cra() {
        let template = get_toolchain_template("cra");
        let expected = "npx create-react-app NAME --template typescript";
        assert_eq!(template, expected);
    }

    #[test]
    fn test_get_toolchain_template_vite() {
        let template = get_toolchain_template("vite");
        let expected = "npm create vite@latest NAME -- --template=react-ts";
        assert_eq!(template, expected);
    }

    #[test]
    fn test_get_toolchain_template_empty_string() {
        let template = get_toolchain_template("");
        let expected = "npm create vite@latest NAME -- --template=react-ts";
        assert_eq!(template, expected);
    }

    #[test]
    fn test_get_toolchain_template_default() {
        let template = get_toolchain_template("test");
        let expected = "npm create vite@latest NAME -- --template=react-ts";
        assert_eq!(template, expected);
    }
}
