# suggestion [![crates.io version](https://img.shields.io/crates/v/suggestion.svg)](https://crates.io/crates/suggestion) [![crates.io downloads](https://img.shields.io/crates/d/suggestion.svg)](https://crates.io/crates/suggestion)

A minimal library for similar name suggestions to provide helps like "Did you mean?"
This library provides suggestion traits for all collection types in the standard library.

## Example

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

## Supported types

Please let me know by issues or pull requests if there is anything left out.

### Sequences

* `LinkedList`
* `VecDeque`
* `Vec`

### Maps

* `HashMap`
* `BTreeMap`

### Sets

* `BTreeSet`
* `HashSet`

### Misc

* `BinaryHeap`
* `[T; N]`: primitive array
* `[T]`: slices

## Contribution

Contributions, including issues and pull requests, are very welcome.
