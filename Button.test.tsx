import { render, screen } from '@testing-library/react';
import Button from './Button';

describe('Button tests', () => {
  test('Button renders', () => {
    render(<Button />);
  });
});
