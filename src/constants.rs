pub const COMMANDS_HELPER: &str = "Available commands:
na: New App (new app)
nc: New component (new component)
nh: New hook (new hook)
lc: Adds eslint and vscode settings and snippets to an existing app (lint and code)
eslint: Adds eslint to an existing app (eslint)
vscode: Adds vscode settings and snippets to an existing app (vscode)
";

pub const ESLINT: &str = "module.exports = {
  env: {
    browser: true,
    es2020: true,
    node: true,
    jest: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:react/recommended',
    'plugin:react/jsx-runtime',
    'plugin:react-hooks/recommended',
    'plugin:jest/recommended',
    'plugin:jest/style',
    'prettier',
  ],
  overrides: [
    {
      files: ['*.jsx', '*.tsx'],
      excludedFiles: '*.test.*',
      rules: {
        'import/prefer-default-export': 1,
      },
    },
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 11,
    sourceType: 'module',
  },
  plugins: [
    '@typescript-eslint',
    'react',
    'react-hooks',
    'prettier',
    'import',
    'jest',
  ],
  rules: {
    'consistent-return': 0,
    'import/export': 2,
    'no-console': 1,
    'no-nested-ternary': 0,
    'no-param-reassign': 0,
    'no-underscore-dangle': 0,
    'prettier/prettier': 'error',
    'react/jsx-filename-extension': [1, { extensions: ['.tsx', '.jsx'] }],
    'react/jsx-props-no-spreading': 0,
    'react/function-component-definition': [
      2,
      { namedComponents: 'arrow-function' },
    ],
    'react/require-default-props': 0,
    'react-hooks/exhaustive-deps': 'warn',
    'react-hooks/rules-of-hooks': 'error',
  },
  settings: {
    'import/resolver': {
      typescript: {},
    },
    react: {
      version: 'detect',
    },
  },
};
";

pub const ESLINT_IGNORE: &str = "*/.js
build
dist
node_modules/
.npm
.eslintcache
*.tgz
.env
.env.*
.vscode/*
.history/
";

pub const NPM_I: &str = "npm i";

pub const NPM_I_DEV: &str = "npm i -D";

pub const LINT_DEPENDENCIES: [&str; 10] = [
    "prettier",
    "eslint",
    "eslint-plugin-react",
    "eslint-plugin-react-hooks",
    "eslint-config-prettier",
    "eslint-plugin-prettier",
    "eslint-import-resolver-typescript",
    "eslint-plugin-import",
    "eslint-plugin-jest",
    "@typescript-eslint/eslint-plugin",
];

pub const DEPENDENCIES: [&str; 10] = [
    "prettier",
    "eslint",
    "eslint-plugin-react",
    "eslint-plugin-react-hooks",
    "eslint-config-prettier",
    "eslint-plugin-prettier",
    "eslint-import-resolver-typescript",
    "eslint-plugin-import",
    "eslint-plugin-jest",
    "@typescript-eslint/eslint-plugin",
];

pub const VITE_DEPS: [&str; 15] = [
    "prettier",
    "eslint",
    "eslint-plugin-react",
    "eslint-plugin-react-hooks",
    "eslint-config-prettier",
    "eslint-plugin-prettier",
    "eslint-import-resolver-typescript",
    "eslint-plugin-import",
    "@typescript-eslint/eslint-plugin",
    "@testing-library/jest-dom",
    "@testing-library/react",
    "@testing-library/user-event",
    "eslint-plugin-vitest",
    "jsdom",
    "vitest",
];

pub const PRETTIER: &str = r#"{
    "arrowParens": "avoid",
    "endOfLine": "auto",
    "jsxSingleQuote": true,
    "printWidth": 80,
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5",
    "tabWidth": 2
}
"#;

pub const VSCODE_SETTINGS: &str = r#"{
  "editor.minimap.enabled": false,
  "editor.mouseWheelZoom": true,
  "editor.trimAutoWhitespace": true,
  "files.trimTrailingWhitespace": true,
  "editor.insertSpaces": true,
  "[typescript]": {
    "editor.formatOnSave": true,
    "editor.tabSize": 2
  },
  "[typescriptreact]": {
    "editor.formatOnSave": true,
    "editor.tabSize": 2
  },
  "[javascript]": {
    "editor.formatOnSave": true,
    "editor.tabSize": 2
  },
  "[javascriptreact]": {
    "editor.formatOnSave": true,
    "editor.tabSize": 2
  }
}
"#;

pub const VSCODE_SNIPPETS: &str = r#"{
  "React TS useState": {
    "scope": "typescript,typescriptreact",
    "prefix": "us",
    "body": [
      "const [${1}, set${1/(.*)/${1:/capitalize}/}] = useState<${2}>(${3});"
    ],
    "description": "React's useState with TypeScript syntax"
  },
  "React TS useEffect": {
    "scope": "typescript,typescriptreact",
    "prefix": "ue",
    "body": ["useEffect(() => {${1}}, []);"],
    "description": "React's useEffect with TypeScript syntax"
  },
  "React TS new functional component": {
    "scope": "typescript,typescriptreact",
    "prefix": "rnfc",
    "body": ["const ${1} = () => {};", "", "export default ${1};"],
    "description": "React's useState with TypeScript syntax"
  },
  "New arrow function": {
    "scope": "typescript,typescriptreact",
    "prefix": "naf",
    "body": ["const ${1} = () => {};"],
    "description": "New arrow function"
  },
  "Jest new file": {
    "scope": "typescript,typescriptreact",
    "prefix": "jnf",
    "body": ["describe('${1}', () => {", "  test('${2}', () => {});", "});"],
    "description": "Jest new test file with describe and empty test"
  },
  "Jest new react file": {
    "prefix": "jnrf",
    "body": [
      "import { render, screen } from '@testing-library/react';",
      "",
      "describe('${1}', () => {",
      "  test('${2}', () => {});",
      "});"
    ],
    "description": "Jest react test file with imports, describe and empty test"
  },
  "Jest new test": {
    "scope": "typescript,typescriptreact",
    "prefix": "jnt",
    "body": ["test('${1}', () => {});"],
    "description": "Jest new test function with describe and empty test"
  },
  "Jest new async test": {
    "scope": "typescript,typescriptreact",
    "prefix": "jnat",
    "body": ["test('${1}', async () => {});"],
    "description": "Jest new async test function with describe and empty test"
  }
}
"#;
