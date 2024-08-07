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

pub const STORIES: &str = "import type { Meta, StoryObj } from '@storybook/react';
import NAME from './NAME';

const meta: Meta<typeof NAME> = {
  component: NAME,
};

export default meta;

type Story = StoryObj<typeof NAME>;

export const Primary: Story = {
  args: {},
  render: (args) => <NAME {...args} />,
};
";

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
