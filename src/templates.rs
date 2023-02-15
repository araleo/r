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
