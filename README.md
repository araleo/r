# R

R is a CLI to automate some of my React workflow.

## Features

- [x] Creates a new functional component
- [x] Creates a new test file
- [x] Finds the components folder automatically
- [x] Creates a test file in a user input folder in the proper subfolder (mirroring the apps folder structure)

## Quickstart

To create a new functional component in the components subfolder in a subfolder and in its own subfolder

```
r nfc -n <name> -c -f -d <folder>
```

To create a new functional component in the components subfolder in a subfolder and in its own subfolder and the test file in a different base path

```
r nfc -n <name> -c -f -d <folder> -t <test_path>
```

To create a new functional component

```
r nfc -n <name>
```

To create a new functional component in the components subfolder

```
r nfc -n <name> -c
```

To create a new functional component in the components subfolder in its own subfolder

```
r nfc -n <name> -c -f
```

### Available commands

- nfc: creates a new functional component
- ntf: creates a new test file

### Available options

- -n --name: Name of the component
- -d --dir: Target directory

### Available flags

- -s --skip-test: Avoid creating a test file when creating a new component
- -c --component: Finds a components folder to use as base dir
- -f --folder: Creates the component in a folder with the same name
