pub const ESLINT: &str = r#"module.exports = {
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
      'prettier',
    ],
    parser: '@typescript-eslint/parser',
    parserOptions: {
      ecmaFeatures: {
        jsx: true,
      },
      ecmaVersion: 11,
      sourceType: 'module',
    },
    plugins: ['@typescript-eslint', 'react', 'react-hooks', 'prettier'],
    rules: {
      'prettier/prettier': 'error',
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
      'import/extensions': 0,
      'react/jsx-filename-extension': [1, { extensions: ['.tsx', '.jsx'] }],
      'no-console': [2, { allow: ['warn', 'error'] }],
      'no-underscore-dangle': 0,
      'consistent-return': 0,
      'no-param-reassign': 0,
      'no-nested-ternary': 0,
      'react/jsx-props-no-spreading': 0,
      'react/require-default-props': 0,
      'react/function-component-definition': [
        2,
        { namedComponents: 'arrow-function' },
      ],
    },
    settings: {
      'import/resolver': {
        typescript: {},
      },
      react: {
        version: 'detect',
      },
    },
  };"#;

pub const NPM_I: &str = "npm install";

pub const NPM_I_DEPS: &str =
    "npm install -D prettier eslint eslint-plugin-react eslint-plugin-react-hooks eslint-config-prettier";

pub const PRETTIER: &str = r#"{
    "arrowParens": "avoid",
    "endOfLine": "auto",
    "jsxSingleQuote": true,
    "printWidth": 80,
    "semi": true,
    "singleQuote": true,
    "trailingComma": "es5",
    "tabWidth": 2
}"#;
