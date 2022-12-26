# R

R is a CLI to automate some of my React workflow.

## Features

- [x] Creates a new functional component
- [x] Creates a new test file
- [ ] Finds the components folder automatically
- [ ] Creates a test file in a \_\_tests\_\_ folder in the proper subfolder (mirroring the apps folder structure)

## Usage

```
r <command> -o --options
```

### Available commands

- nfc: creates a new functional component
- ntf: creates a new test file

### Available flags and options

- -n --name: Name of the component
- -d --dir: Target directory
- -s --skip-test: Avoid creating a test file when creating a new component
