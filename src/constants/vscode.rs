const VSCODE_SETTINGS: &str = r#"{
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

pub fn get_settings() -> String {
    VSCODE_SETTINGS.to_string()
}

const VSCODE_SNIPPETS: &str = r#"{
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

pub fn get_snippets() -> String {
    VSCODE_SNIPPETS.to_string()
}
