# Cute

[![Build Status](https://travis-ci.org/mattgathu/cute.svg?branch=master)](https://travis-ci.org/mattgathu/cute)
[![Build status](https://ci.appveyor.com/api/projects/status/pt592bgkx2mkt56m?svg=true)](https://ci.appveyor.com/project/mattgathu/cute)
[![Crates.io](https://img.shields.io/crates/v/cute.svg)](https://crates.io/crates/cute)
[![docs.rs](https://docs.rs/cute/badge.svg)](https://docs.rs/cute)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Macro for Python-esque list comprehensions in Rust.

 The `c!` macro implements list and hashmap comprehensions similar to those found in Python,
 allowing for conditionals and nested comprehensions.

 # Python Syntax

 ```python
 squares = [x*x for x in range(10)]

 even_squares = [x*x for x in range(10) if x % 2 == 0]

 squares_dict = {key:key*key for key in range(10)}
 ```

 # c! Syntax

 ```rust
 let squares = c![x*x, for x in 0..10];

 let even_squares = c![x*x, for x in 0..10, if x % 2 == 0];
 
 let squares_hashmap = c!{key => key*key, for key in 0..10};

 ```

 Note `c!`'s has the comprehension's parts, comma-separated.

 # Examples

 Simpe comprehension

 ```rust
 #[macro_use(c)]
 extern crate cute;

 let v = [1,2,3,4];
 let v_squared = c![x*x, for x in v];

 ```
 Conditional filtering

 ```rust
 let squares = c![x*x, for x in 0..10, if x % 2 == 0];
 assert_eq!(squares, vec![0, 4, 16, 36, 64]);
 ```
 
 Nested Comprehensions

 ```rust
 let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
 let flat: Vec<usize> = c![x, for x in y, for y in nested];
 assert_eq!(flat, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
 ``` 
 
 ```rust
 let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
 let even_flat: Vec<usize> = c![x, for x in y, for y in nested, if x % 2 == 0];
 assert_eq!(even_flat, vec![2, 4, 6, 8]);
 ```

 Comprehensions over Iterators

 ```rust
 let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
 let output: Vec<i32> = c![x*2, for x in vec.iter()];
 assert_eq!(output, vec![-8, -4, 0, 4, 8]);
 ``` 
 
 ```rust
 let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
 let output: Vec<i32> = c![x, for x in vec.iter(), if *x >= 0i32];
 assert_eq!(output, vec![0, 2, 4]);
 ``` 
 
 Function Application

 ```rust
 let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
 let output: Vec<i32> = c![x.abs(), for x in vec.iter()];
 assert_eq!(output, vec![4, 2, 0, 2, 4]);
 ```

 ```rust
 fn square(x:i32) -> i32 {
        x*x
 }
       
 let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
 let squares: Vec<i32> = c![square(x), for x in vec];
 assert_eq!(squares, vec![16, 4, 0, 4, 16]);
 ```
 
HashMap Comprehensions (Dictionary Comprehensions)

```rust
// simple hashmap comprehension

let squares_hashmap = c!{key => key*key, for key in 0..10};

```

```rust
// hashmap comprehension from an Iterator
// NOTE: we have to perform dereferencing: *key

let map = c!{*key => key*key, for key in vec![1,2].iter()};

```

```rust
// conditional hashmap comprehension

let v: Vec<(&str, i32)> = vec![("one", 1), ("two", 2), ("three", 3)];
let map = c!{key => val, for (key, val) in v, if val == 1 || val == 2};

let mut expected: HashMap<&str, i32> = HashMap::new();
expected.insert("one", 1);
expected.insert("two", 2);

assert_eq!(map, expected);
```

```rust
// conditional hashmap comprehension from an Iterator
// NOTE: we perform deferencing when using values

let map = c!{*key => key*key, for key in vec![1,2].iter(), if *key % 2 == 0};
let mut e: HashMap<i32, i32> = HashMap::new();
e.insert(2, 4);

assert_eq!(map, e);
```



