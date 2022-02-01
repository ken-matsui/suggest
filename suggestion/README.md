# suggestion [![crates.io version](https://img.shields.io/crates/v/suggestion.svg)](https://crates.io/crates/suggestion) [![crates.io downloads](https://img.shields.io/crates/d/suggestion.svg)](https://crates.io/crates/suggestion)

A minimal library for similar name suggestions to provide helps like "Did you mean?"
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

## Contribution

Contributions, including issues and pull requests, are very welcome.
