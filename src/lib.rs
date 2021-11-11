//! This library provides suggestion traits for all collection types in the standard library.
//!
//! # Example
//! ```
//! use suggestion::Suggest;
//!
//! fn main() {
//!     let input = "instakk";
//!
//!     let list_commands = vec!["update", "install"];
//!     if list_commands.contains(&input) {
//!         return;
//!     }
//!
//!     if let Some(sugg) = list_commands.suggest(input) {
//!         println!("No command named `{}` found.", input);
//!         println!("Did you mean `{}`?", sugg);
//!     }
//! }
//! ```
//!
//! ```console
//! $ cargo run
//! No command named `instakk` found.
//! Did you mean `install`?
//! ```

use lev_distance::find_best_match_for_name;

// Sequences
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::vec::Vec;

// Maps
use std::collections::hash_map::HashMap;
use std::collections::BTreeMap;

// Sets
use std::collections::BTreeSet;
use std::collections::HashSet;

// Misc
use std::collections::BinaryHeap;

pub trait Suggest {
    /// Find similar name in values for all collections
    fn suggest(&self, query: &str) -> Option<String>;
}
pub trait SuggestKey {
    /// Find similar name in keys for Map collections
    fn suggest_key(&self, query: &str) -> Option<String>;
}

macro_rules! impl_suggest {
    ($t:ident) => {
        impl<T: std::convert::AsRef<str>> Suggest for $t<T> {
            fn suggest(&self, query: &str) -> Option<String> {
                find_best_match_for_name(self.iter(), query, None)
            }
        }
    };
}
macro_rules! impl_suggest_key {
    ($t:ident) => {
        impl<T: std::convert::AsRef<str>, U> SuggestKey for $t<T, U> {
            fn suggest_key(&self, query: &str) -> Option<String> {
                find_best_match_for_name(self.keys(), query, None)
            }
        }
    };
}
macro_rules! impl_suggest_value {
    ($t:ident) => {
        impl<T, U: std::convert::AsRef<str>> Suggest for $t<T, U> {
            fn suggest(&self, query: &str) -> Option<String> {
                find_best_match_for_name(self.values(), query, None)
            }
        }
    };
}

// Sequences
impl_suggest!(LinkedList);
impl_suggest!(VecDeque);
impl_suggest!(Vec);

// Maps
impl_suggest_key!(HashMap);
impl_suggest_value!(HashMap);
impl_suggest_key!(BTreeMap);
impl_suggest_value!(BTreeMap);

// Sets
impl_suggest!(BTreeSet);
impl_suggest!(HashSet);

// Misc
impl_suggest!(BinaryHeap);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_suggest {
        ($t:ident, $f:ident) => {
            #[test]
            fn $f() {
                let input: $t<_> = vec!["aaab", "aaabc"].into_iter().collect();
                assert_eq!(input.suggest("aaaa"), Some("aaab".to_string()));
            }
        };
    }
    macro_rules! test_suggest_map {
        ($t:ident, $f:ident) => {
            #[test]
            fn $f() {
                use std::array::IntoIter;

                let input = $t::<_, _>::from_iter(IntoIter::new([("aaab", 2), ("aaabc", 4)]));
                assert_eq!(input.suggest_key("aaaa"), Some("aaab".to_string()));

                let input = $t::<_, _>::from_iter(IntoIter::new([(2, "aaab"), (4, "aaabc")]));
                assert_eq!(input.suggest("aaaa"), Some("aaab".to_string()));
            }
        };
    }

    // Sequences
    test_suggest!(LinkedList, test_suggest_linked_list);
    test_suggest!(VecDeque, test_suggest_vec_deque);
    test_suggest!(Vec, test_suggest_vec);

    // Maps
    test_suggest_map!(HashMap, test_suggest_hash_map);
    test_suggest_map!(BTreeMap, test_suggest_b_tree_map);

    // Sets
    test_suggest!(HashSet, test_suggest_b_tree_set);
    test_suggest!(HashSet, test_suggest_hash_set);

    // Misc
    test_suggest!(BinaryHeap, test_suggest_binary_heap);
}
