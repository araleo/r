# R

R is a CLI to automate some of my React workflow.

## Features

- [x] Creates a new functional component
- [x] Creates a new test file
- [x] Finds the components folder automatically
- [ ] Creates a test file in a \_\_tests\_\_ folder in the proper subfolder (mirroring the apps folder structure)

## Quickstart

To create a new functional component

```
r nfc -n <Name>
```

To create a new functional component in the components subfolder

```
r nfc -n <Name> -c
```

To create a new functional component in the components subfolder in its own subfolder

```
r nfc -n <Name> -c
```

### Available commands

- nfc: creates a new functional component
- ntf: creates a new test file

### Available flags and options

- -n --name: Name of the component
- -d --dir: Target directory
- -s --skip-test: Avoid creating a test file when creating a new component
