# suggestion-cli [![crates.io version](https://img.shields.io/crates/v/suggestion-cli.svg)](https://crates.io/crates/suggestion-cli) [![crates.io downloads](https://img.shields.io/crates/d/suggestion-cli.svg)](https://crates.io/crates/suggestion-cli)

A CLI tool for similar name suggestions to provide helps like "Did you mean?"

The library version is placed [here](./suggestion).

## Installation

```bash
cargo install suggestion-cli
```

## Usage

```bash
$ suggest --help
suggestion-cli 0.1.0
A CLI tool for similar name suggestions to provide helps like "Did you mean?"

USAGE:
    suggest <INPUT> [POSSIBLE_VALUES]...

ARGS:
    <INPUT>                 Input to check if similar name exists
    <POSSIBLE_VALUES>...    Values of similar names

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Examples

```bash
$ suggest instakk update install
The `instakk` input is similar to `install`.

$ suggest hoge update install
No similar name for the `hoge` input was found.

$ suggest install update install
The same value with the `install` input exists.
```

## Contribution

Contributions, including issues and pull requests, are very welcome.
