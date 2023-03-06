const ESLINT_CRA: &str = "module.exports = {
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

pub fn get_config() -> String {
    ESLINT_CRA.to_string()
}

const ESLINT_IGNORE: &str = "*/.js
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

pub fn get_ignore() -> String {
    ESLINT_IGNORE.to_string()
}
