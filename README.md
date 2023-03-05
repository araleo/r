# R

R is a CLI to automate some of my React workflow.

## Features

- [x] Creates a new react app (vite or cra) with eslint, prettier and vscode settings and snippets
- [x] Creates a new functional component
- [x] Creates a new hook
- [x] Creates a new test file
- [x] Finds the components/hooks folder automatically
- [x] Adds eslint and prettier to an existing app
- [x] Adds vscode settings and snippets to an existing app
- [ ] Inits storybook in a created app
- [ ] Adds components library to created app (styled components, mui, etc)
- [ ] Yarn support
- [ ] Full Vite + Vitest support
- [ ] Supports different profiles for configuration files and dependencies (eg. work, personal)
- [ ] Create new profiles using the CLI
- [ ] Parsers eslint file to figure out which dependencies to install

## Quickstart

To create a new react app with vite or cra, eslint and prettier:

You can ommit the -t option and r will use vite as the default toolchain.

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

To add eslint to an existing app (command must be used at the app's root folder)

```
r eslint
```

To add vscode settings and snippets to an existing app (command must be used at the app's root folder)

```
r vscode
```

To add eslint and vscode settings and snippets to an existing app (command must be used at the app's root folder)

```
r lc
```

### Available commands

- ca: New App (create app)
- nc: New component (new component)
- nh: New hook (new hook)
- lc: Adds eslint and vscode settings and snippets to an existing app (lint and code)
- eslint: Adds eslint to an existing app (eslint)
- vscode: Adds vscode settings and snippets to an existing app (vscode)

### Available options

- -n --name: Name of the app / component / hook file
- -d --dir: Target directory
- -r --root: Overrides the base directory to be used (defaults to components or hooks)
- -t --toolchain: Specifies the toolchain to be used when creating a new app (defaults to vite)

### Available flags

- -s --skip-test: Avoids creating a test file when creating a new component or hook
- -f --flat: Avoids creating a new folder to contain the component or hook

### Create app

The ca command will create a new react app with vite or cra and the typescript template. It will run one of the following commands, substituting {NAME} for the given parameter:

`npm create vite@latest NAME -- --template=react-ts`

`npx create-react-app NAME --template typescript`

It will also install eslint, and some eslint plugins, and create eslint and prettier config files and vscode settings and snippets files to the new app.

This command was not tested and will not work on MacOS.
