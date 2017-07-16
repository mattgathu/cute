#![doc(html_logo_url = "https://raw.githubusercontent.com/mattgathu/cute/master/C!.png")]
//! A Macro for python-esque list and dictionary(hashmap) comprehensions in Rust
//!
//! The `c!` macro implements list and hashmap comprehensions similar to those found in Python,
//! allowing for conditionals and nested comprehensions.
//!
//! # Python Syntax
//! ```
//! squares = [x*x for x in range(10)]
//!
//! even_squares = [x*x for x in range(10) if x % 2 == 0]
//! ```
//!
//! # c! Syntax
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let squares = c![x*x, for x in 0..10];
//!
//! let even_squares = c![x*x, for x in 0..10, if x % 2 == 0];
//!
//! ```
//!
//! `c!`'s has the comprehension's parts, comma-separated.
//!
//! # Examples
//!
//! Simple comprehension
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let v = [1,2,3,4];
//! let v_squared = c![x*x, for x in v];
//!
//! ```
//! Conditional filtering
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let squares = c![x*x, for x in 0..10, if x % 2 == 0];
//! assert_eq!(squares, vec![0, 4, 16, 36, 64]);
//! ```
//!
//! Nested Comprehensions
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
//! let flat: Vec<usize> = c![x, for x in y, for y in nested];
//! assert_eq!(flat, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
//! ```
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
//! let even_flat: Vec<usize> = c![x, for x in y, for y in nested, if x % 2 == 0];
//! assert_eq!(even_flat, vec![2, 4, 6, 8]);
//! ```
//!
//! Comprehensions over Iterators
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
//! let output: Vec<i32> = c![x*2, for x in vec.iter()];
//! assert_eq!(output, vec![-8, -4, 0, 4, 8]);
//! ```
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
//! let output: Vec<i32> = c![x, for x in vec.iter(), if *x >= 0i32];
//! assert_eq!(output, vec![0, 2, 4]);
//! ```
//!
//! Function Application
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
//! let output: Vec<i32> = c![x.abs(), for x in vec.iter()];
//! assert_eq!(output, vec![4, 2, 0, 2, 4]);
//! ```
//!
//! ```
//! #[macro_use(c)]
//! extern crate cute;
//!
//! fn square(x:i32) -> i32 {
//!        x*x
//! }
//!
//! let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
//! let squares: Vec<i32> = c![square(x), for x in vec];
//! assert_eq!(squares, vec![16, 4, 0, 4, 16]);
//! ```
//!
//! Simple Hashmap creation Comprehension
//!
//! ```
//! let v = vec!["one", "two", "three"];
//! let map = c![key, key.to_uppercase(), for key in v];
//! let mut expected: HashMap<&str, String> = HashMap::new();
//! expected.insert("one", String::from("ONE"));
//! expected.insert("two", String::from("TWO"));
//! expected.insert("three", String::from("THREE"));
//!
//! assert_eq!(map, expected);
//! ```

use std::collections::HashMap;

#[macro_export]
macro_rules! c {

    ($exp:expr, for $i:ident in $iter:expr) => (
        {
            let mut r = vec![];
            for $i in $iter {
                r.push($exp);
            }
            r
        }
    );

    ($exp:expr, for $i:ident in $iter:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i in $iter {
                if $cond {
                    r.push($exp.clone());
                }
            }
            r
        }
    );

    ($exp:expr, for $i:ident in $iter:expr, for $i2:ident in $iter2:expr) => (
        {
            let mut r = vec![];
            for $i2 in $iter2 {
                for $i in $iter {
                    r.push($exp);
                }
            }
            r
        }
    );

    ($exp:expr, for $i:ident in $iter:expr, for $i2:ident in $iter2:expr, if $cond:expr) => (
        {
            let mut r = vec![];
            for $i2 in $iter2 {
                for $i in $iter {
                    if $cond{
                        r.push($exp);
                    }
                }
            }
            r
        }
    );

    ($key:expr, $val:expr, for $i:ident in $iter:expr) => (
        {
            let mut map = HashMap::new();
            for $i in $iter {
                map.insert($key, $val);
            }
            map
        }
    );
}


#[test]
fn simple_comprehension() {
    let squares: Vec<usize> = c![x*x, for x in 0..10];
    assert_eq!(squares, vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
}

#[test]
fn filter_comprehension() {
    let squares = c![x*x, for x in 0..10, if x % 2 == 0];
    assert_eq!(squares, vec![0, 4, 16, 36, 64]);
}

#[test]
fn simple_nested_comprehension() {
    let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let flat: Vec<usize> = c![x, for x in y, for y in nested];
    assert_eq!(flat, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn filter_nested_comprehension() {
    let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let even_flat: Vec<usize> = c![x, for x in y, for y in nested, if x % 2 == 0];
    assert_eq!(even_flat, vec![2, 4, 6, 8]);
}


#[test]
fn vector_to_iter_comprehension() {
    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let output: Vec<i32> = c![x*2, for x in vec.iter()];
    assert_eq!(output, vec![-8, -4, 0, 4, 8]);
}

#[test]
fn filter_comprehension_two() {
    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let output: Vec<i32> = c![x, for x in vec.iter(), if *x >= 0i32];
    assert_eq!(output, vec![0, 2, 4]);
}

#[test]
fn apply_function_comprehension() {
    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let output: Vec<i32> = c![x.abs(), for x in vec.iter()];
    assert_eq!(output, vec![4, 2, 0, 2, 4]);
}

#[test]
fn apply_user_defined_function() {
    fn square(x:i32) -> i32 {
            x*x
    }

    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let squares: Vec<i32> = c![square(x), for x in vec];
    assert_eq!(squares, vec![16, 4, 0, 4, 16]);

}

#[test]
fn hashmap_comprehension() {
    let v = vec!["one", "two", "three"];
    let map = c![key, key.to_uppercase(), for key in v];
    let mut expected: HashMap<&str, String> = HashMap::new();
    expected.insert("one", String::from("ONE"));
    expected.insert("two", String::from("TWO"));
    expected.insert("three", String::from("THREE"));

    assert_eq!(map, expected);

}
