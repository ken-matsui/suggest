# suggestion

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
