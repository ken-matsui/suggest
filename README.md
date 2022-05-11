# suggest [![crates.io version](https://img.shields.io/crates/v/suggest.svg)](https://crates.io/crates/suggest) [![crates.io downloads](https://img.shields.io/crates/d/suggest.svg)](https://crates.io/crates/suggest)

A minimal library & CLI tool to provide similar name suggestions like "Did you mean?"
This library provides suggestion traits for all collection types in the standard library.
A WebAssembly package is also supported.

This library is intended to suggest a candidate from a list of unknown suggestions until runtime, in addition to the suggestion feature already available in [`clap`](https://github.com/clap-rs/clap#default-features).

> This crate was previously called [`suggestion`](https://crates.io/crates/suggestion) but renamed, and it'll be yanked. 

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
suggest = "0.4"
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

To suggest keys, use `suggest::SuggestKey` trait.

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
cargo install suggest
```

#### WebAssembly

This application also provides a wasm package.
You can install it using [`wapm`](https://wapm.io/help/install) by the following command:

```bash
$ wapm install ken-matsui/suggest
```

### Usage

```bash
$ suggest --help
suggest 0.3.1
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

#### WebAssembly

```bash
$ wapm run suggest --help
...
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

#### WebAssembly

```bash
$ wapm run suggest instakk update install
The `instakk` input is similar to `install`.

$ wapm run suggest hoge update install
No similar name for the `hoge` input was found.

$ wapm run suggest install update install
The same value with the `install` input exists.

$ wapm run suggest paoc poac poacpp
No similar name for the `paoc` input was found.

$ wapm run suggest paoc poac poacpp --distance 2
The `paoc` input is similar to `poac`.
```

## Contribution

Contributions, including issues and pull requests, are very welcome.

### Build

```bash
$ cargo build
```

Or you can directly execute the binary:

```bash
$ cargo run
```

#### WebAssembly

```bash
$ rustup target add wasm32-wasi
$ cargo build --target wasm32-wasi
$ wasmer run target/wasm32-wasi/debug/suggest.wasm encode hello
```

### Test

This command can also test C API.

```bash
$ cargo build
$ cargo test
```

### Publish

#### [GitHub Releases](https://github.com/ken-matsui/base64-cli/tags)

```bash
$ git tag v0.1.0
$ git push origin v0.1.0
```

#### [crates.io](https://crates.io/)

```bash
$ cargo publish
```

#### [wapm.io](https://wapm.io/)

```bash
$ cargo build --release --target wasm32-wasi
$ wapm publish
```
