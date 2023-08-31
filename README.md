# R

R is a CLI to automate some of my React workflow.

## This software is unfinished and under development

## Features

- [x] Creates a new functional component
- [x] Creates a new hook
- [x] Creates a new test file
- [x] Finds the components/hooks folder automatically

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

- nc: New component (new component)
- nh: New hook (new hook)

### Available options

- -n --name: Name of the app / component / hook file
- -d --dir: Target directory
- -r --root: Overrides the base directory to be used (defaults to components or hooks)

### Available flags

- -s --skip-test: Avoids creating a test file when creating a new component or hook
- -f --flat: Avoids creating a new folder to contain the component or hook
