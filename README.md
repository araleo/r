# R

R is a CLI to automate some of my React workflow.

## Features

- [x] Creates a new react app (vite or cra) with eslint and prettier
- [x] Creates a new functional component
- [x] Creates a new hook
- [x] Creates a new test file
- [x] Finds the components/hooks folder automatically

## Quickstart

To create a new react app with vite or cra, eslint and prettier:

```
r ca -n <name> -t <toolchain>
```

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

- ca: creates a new react app with vite or cra
- nc: creates a new functional component
- nh: creates a new hook

### Available options

- -n --name: Name of the component / hook / test file
- -d --dir: Target directory

### Available flags

- -s --skip-test: Avoids creating a test file when creating a new component or hook
- -f --flat: Avoids creating a new folder to contain the component or hook

### Create app

The ca command will create a new react app with vite or cra and the typescript template. It will run one of the following commands, substituting {NAME} for the given parameter:

`npm create vite@latest NAME -- --template=react-ts`

`npx create-react-app NAME --template typescript`

It will also install eslint and create eslint and prettier config files to the new app. You can edit the vite or cra command and the configuration files in the templates or constants module.

This command was not tested on MacOS.
