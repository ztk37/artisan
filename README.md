> `Disclaimer:` This project is in an early stage, expect bugs, missing features and unannounced changes.

![logo](./.github/logo.png)

> `Artisan` is a command line application for generating projects based on a single [toml](https://toml.io/en/) file.

## Features

- `Generate` new projects based on a self-contained template file written in [TOML](https://toml.io/en/). (WIP)

## Usage

```
Usage: artisan <COMMAND>

Commands:
  new     Create a new project from a template
  config  Manage artisans global config
  home    Manage artisans home directory
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Future Work

- `Hooks` integrate git hooks (considering)
- `Manage` your templates directly from the command line. (considering)
- `Release` your generated projects directly to GitHub. (considering)
