# suggest [![crates.io version](https://img.shields.io/crates/v/suggest.svg)](https://crates.io/crates/suggest) [![crates.io downloads](https://img.shields.io/crates/d/suggest.svg)](https://crates.io/crates/suggest)

A minimal library to provide similar name suggestions like "Did you mean?"
This library provides suggestion traits for all collection types in the standard library.

This library is intended to suggest a candidate from a list of unknown suggestions until runtime, in addition to the suggestion feature already available in [`clap`](https://github.com/clap-rs/clap#default-features).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
suggest = "0.5"
```

## Examples

### Simple case

This example can be executed by the `cargo run --example simple` command.

```rust
use suggest::Suggest;

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
use suggest::Suggest;

fn main() {
    let input = "paoc";

    let list_commands = vec!["poac", "poacpp"];
    if list_commands.contains(&input) {
        return;
    }

    if let Some(sugg) = list_commands.suggest_by(input, 2) {
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

To suggest keys, use `suggest::SuggestKey` trait.

### Sets

* `BTreeSet`
* `HashSet`

### Misc

* `BinaryHeap`
* `[T; N]`: primitive array
* `[T]`: slices

## Contribution

Contributions, including issues and pull requests, are very welcome.

### Build

```bash
$ cargo build
```

### Test

This command can also test C API.

```bash
$ cargo build
$ cargo test
```

### Publish

#### [GitHub Releases](https://github.com/ken-matsui/suggest/tags)

```bash
$ git tag v0.1.0
$ git push origin v0.1.0
```

#### [crates.io](https://crates.io/)

```bash
$ cargo publish
```
