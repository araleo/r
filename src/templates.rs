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
