pub mod dependencies;
pub mod eslint;
pub mod prettier;
pub mod vscode;

pub const DEFAULT_PROFILE: &str = "default";

pub const COMMANDS_HELPER: &str = "Available commands:
na: New App (new app)
nc: New component (new component)
nh: New hook (new hook)
lc: Adds eslint and vscode settings and snippets to an existing app (lint and code)
eslint: Adds eslint to an existing app (eslint)
vscode: Adds vscode settings and snippets to an existing app (vscode)
";

pub const IGNORED_DIRS: [&str; 8] = [
    "node_modules",
    "dist",
    "__tests__",
    "tests",
    ".git",
    ".vscode",
    "coverage",
    "public",
];
