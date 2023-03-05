const PRETTIER: &str = r#"{
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

pub fn get_config() -> String {
    PRETTIER.to_string()
}
