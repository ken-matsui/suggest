# suggestion [![crates.io version](https://img.shields.io/crates/v/suggestion.svg)](https://crates.io/crates/suggestion) [![crates.io downloads](https://img.shields.io/crates/d/suggestion.svg)](https://crates.io/crates/suggestion)

A minimal library & CLI tool to provide similar name suggestions like "Did you mean?"
This library provides suggestion traits for all collection types in the standard library.

## Examples

### Simple case

This example can be executed by the `cargo run --example simple` command.

```rust
use suggestion::Suggest;

fn main() {
    let input = "instakk";

    let list_commands = vec!["update", "install"];
    if list_commands.contains(&input) {
        return;
    }

    if let Some(sugg) = list_commands.suggest(input) {
        println!("No command named `{}` found.", input);
        println!("Did you mean `{}`?", sugg);
    }
}
```

```shell
$ cargo run
No command named `instakk` found.
Did you mean `install`?
```

### Specifying distance

```rust
use suggestion::Suggest;

fn main() {
    let input = "paoc";

    let list_commands = vec!["poac", "poacpp"];
    if list_commands.contains(&input) {
        return;
    }

    if let Some(sugg) = list_commands.suggest_with_dist(input, Some(2)) {
        println!("No command named `{}` found.", input);
        println!("Did you mean `{}`?", sugg);
    }
}
```

```shell
$ cargo run
No command named `paoc` found.
Did you mean `poac`?
```

## Supported types

Please let me know if anything is left out through issues or pull requests.

### Sequences

* `LinkedList`
* `VecDeque`
* `Vec`

### Maps

* `HashMap`
* `BTreeMap`

To suggest keys, use `suggestion::SuggestKey` trait.

### Sets

* `BTreeSet`
* `HashSet`

### Misc

* `BinaryHeap`
* `[T; N]`: primitive array
* `[T]`: slices

## CLI

### Installation

```bash
cargo install suggestion
```

### Usage

```bash
$ suggest --help
suggestion 0.3.1
A minimal library & CLI tool to provide similar name suggestions like "Did you mean?"

USAGE:
    suggest [OPTIONS] <INPUT> [VALUES]...

ARGS:
    <INPUT>        Input to check if similar name exists
    <VALUES>...    Values of similar names

OPTIONS:
    -d, --distance <DISTANCE>    Levenshtein Distance
    -h, --help                   Print help information
    -q, --quiet                  Disable console outputs
    -V, --version                Print version information
```

### Examples

```bash
$ suggest instakk update install
The `instakk` input is similar to `install`.

$ suggest hoge update install
No similar name for the `hoge` input was found.

$ suggest install update install
The same value with the `install` input exists.

$ suggest paoc poac poacpp
No similar name for the `paoc` input was found.

$ suggest paoc poac poacpp --distance 2
The `paoc` input is similar to `poac`.
```

## Contribution

Contributions, including issues and pull requests, are very welcome.
