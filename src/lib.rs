//! This library provides suggestion traits for all collection types in the standard library.
//!
//! # Example
//! ```
//! use suggest::Suggest;
//!
//! let input = "instakk";
//!
//! let list_commands = vec!["update", "install"];
//! if list_commands.contains(&input) {
//!     return;
//! }
//!
//! if let Some(sugg) = list_commands.suggest(input) {
//!     println!("No command named `{}` found.", input);
//!     println!("Did you mean `{}`?", sugg);
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

    /// Find similar name with dist in values for all collections
    fn suggest_by(&self, query: &str, dist: usize) -> Option<String>;
}
pub trait SuggestKey {
    /// Find similar name in keys for Map collections
    fn suggest_key(&self, query: &str) -> Option<String>;

    /// Find similar name with dist in keys for Map collections
    fn suggest_key_by(&self, query: &str, dist: usize) -> Option<String>;
}

macro_rules! impl_suggest_fns {
    ($f:ident) => {
        fn suggest(&self, query: &str) -> Option<String> {
            find_best_match_for_name(self.$f(), query, None)
        }
        fn suggest_by(&self, query: &str, dist: usize) -> Option<String> {
            find_best_match_for_name(self.$f(), query, Some(dist))
        }
    };
}

macro_rules! impl_suggest {
    ($t:ident) => {
        impl<T: AsRef<str>> Suggest for $t<T> {
            impl_suggest_fns!(iter);
        }
    };
}

macro_rules! impl_suggest_value {
    ($t:ident) => {
        impl<T, U: AsRef<str>> Suggest for $t<T, U> {
            impl_suggest_fns!(values);
        }
    };
}
macro_rules! impl_suggest_key {
    ($t:ident) => {
        impl<T: AsRef<str>, U> SuggestKey for $t<T, U> {
            fn suggest_key(&self, query: &str) -> Option<String> {
                find_best_match_for_name(self.keys(), query, None)
            }
            fn suggest_key_by(&self, query: &str, dist: usize) -> Option<String> {
                find_best_match_for_name(self.keys(), query, Some(dist))
            }
        }
    };
}
macro_rules! impl_suggest_map {
    ($t:ident) => {
        impl_suggest_key!($t);
        impl_suggest_value!($t);
    };
}

// Primitive Array Type
impl<T: AsRef<str>, const N: usize> Suggest for [T; N] {
    impl_suggest_fns!(iter);
}

// Slices
impl<T: AsRef<str>> Suggest for [T] {
    impl_suggest_fns!(iter);
}

// Sequences
impl_suggest!(LinkedList);
impl_suggest!(VecDeque);
impl_suggest!(Vec);

// Maps
impl_suggest_map!(HashMap);
impl_suggest_map!(BTreeMap);

// Sets
impl_suggest!(BTreeSet);
impl_suggest!(HashSet);

// Misc
impl_suggest!(BinaryHeap);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_suggest_primitive {
        ($t:ty, $f:ident) => {
            #[test]
            fn $f() {
                let tmp = ["aaab", "aaabc"];
                let input: $t = &tmp;
                assert_eq!(input.suggest("aaaa"), Some("aaab".to_string()));

                let tmp = ["poac", "poacpp"];
                let input: $t = &tmp;
                assert_eq!(input.suggest("paoc"), None);
                assert_eq!(input.suggest_by("paoc", 1), None);
                assert_eq!(input.suggest_by("paoc", 2), Some("poac".to_string()));
            }
        };
    }
    macro_rules! test_suggest {
        ($t:ident, $f:ident) => {
            #[test]
            fn $f() {
                let input: $t<_> = vec!["aaab", "aaabc"].into_iter().collect();
                assert_eq!(input.suggest("aaaa"), Some("aaab".to_string()));

                let input: $t<_> = vec!["poac", "poacpp"].into_iter().collect();
                assert_eq!(input.suggest("paoc"), None);
                assert_eq!(input.suggest_by("paoc", 1), None);
                assert_eq!(input.suggest_by("paoc", 2), Some("poac".to_string()));
            }
        };
    }
    macro_rules! test_suggest_map {
        ($t:ident, $f:ident) => {
            #[test]
            fn $f() {
                let input = $t::<_, _>::from_iter([("aaab", 2), ("aaabc", 4)].into_iter());
                assert_eq!(input.suggest_key("aaaa"), Some("aaab".to_string()));

                let input = $t::<_, _>::from_iter([(2, "aaab"), (4, "aaabc")].into_iter());
                assert_eq!(input.suggest("aaaa"), Some("aaab".to_string()));

                let input = $t::<_, _>::from_iter([("poac", 2), ("poacpp", 4)].into_iter());
                assert_eq!(input.suggest_key("paoc"), None);
                assert_eq!(input.suggest_key_by("paoc", 1), None);
                assert_eq!(input.suggest_key_by("paoc", 2), Some("poac".to_string()));

                let input = $t::<_, _>::from_iter([(2, "poac"), (4, "poacpp")].into_iter());
                assert_eq!(input.suggest("paoc"), None);
                assert_eq!(input.suggest_by("paoc", 1), None);
                assert_eq!(input.suggest_by("paoc", 2), Some("poac".to_string()));
            }
        };
    }

    // Primitive Array Type
    test_suggest_primitive!(&[&str; 2], test_array);
    // Slices
    test_suggest_primitive!(&[&str], test_slices);

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
