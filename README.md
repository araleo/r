# R

R is a CLI to automate some of my React workflow.

## Features

- [x] Creates a new functional component
- [x] Creates a new hook
- [x] Creates a new test file
- [x] Finds the components/hooks/\_\_tests\_\_ folder automatically
- [x] Creates a test file in a user input folder in the proper subfolder (mirroring the apps folder structure)

## Quickstart

To create a new functional component in the components folder and in it's own subfolder

```
r nc -n <name>
```

To create a new functional component in a subdirectory of the components folder and in it's own subfolder

```
r nc -n <name> -d <dir>
```

To create a new hook in the hooks folder and in it's own subfolder

```
r nh -n <name>
```

To create a new hook in a subdirectory of the hooks folder and in it's own subfolder

```
r nh -n <name> -d <dir>
```

### Available commands

- nc: creates a new functional component
- nh: creates a new hook

### Available options

- -n --name: Name of the component / hook / test file
- -d --dir: Target directory

### Available flags

- -s --skip-test: Avoids creating a test file when creating a new component or hook
- -f --flat: Avoids creating a new folder to contain the component or hook
