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
}
