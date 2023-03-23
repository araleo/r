pub const NPM_I: &str = "npm i";

pub const NPM_I_DEV: &str = "npm i -D";

pub const LINT_DEPENDENCIES: [&str; 8] = [
    "prettier",
    "eslint",
    "eslint-plugin-react",
    "eslint-plugin-react-hooks",
    "eslint-config-prettier",
    "eslint-plugin-prettier",
    "eslint-import-resolver-typescript",
    "@typescript-eslint/eslint-plugin",
];

pub const CRA_DEPENDENCIES: [&str; 1] = ["eslint-plugin-jest"];

pub const VITE_DEPS: [&str; 6] = [
    "@testing-library/jest-dom",
    "@testing-library/react",
    "@testing-library/user-event",
    "eslint-plugin-vitest",
    "jsdom",
    "vitest",
];
