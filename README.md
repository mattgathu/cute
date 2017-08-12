# Cute

[![Build Status](https://travis-ci.org/mattgathu/cute.svg?branch=master)](https://travis-ci.org/mattgathu/cute)
[![Build status](https://ci.appveyor.com/api/projects/status/pt592bgkx2mkt56m?svg=true)](https://ci.appveyor.com/project/mattgathu/cute)
[![Crates.io](https://img.shields.io/crates/v/cute.svg)](https://crates.io/crates/cute)
[![docs.rs](https://docs.rs/cute/badge.svg)](https://docs.rs/cute)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Macros for Python-esque list and dictionary (hashmap) comprehensions in Rust.

The `c!` and `i!` macros implements list and hashmap comprehensions
alongside list generators similar to those found in Python, allowing
for conditionals and nested comprehensions.

# Python Syntax Example
```python
even_squares = [x * x for x in range(10) if x % 2 == 0]
squares_dict = {key: key * key for key in range(10)}
even_squares_gen = (x * x for x in range(10) if x % 2 == 0)
```

# `c!` and `i!` Syntax Example
```rust
#[macro_use(c, i)]
extern crate cute;

let even_squares = c![x * x, for x in 0..10, if x % 2 == 0];
let squares_hashmap = c!{key => key * key, for key in 0..10};
let even_squares_iter = i!(x * x, for x in 0..10, if x % 2 == 0);
```

`c!` and `i!` have the comprehension's parts, comma-separated.
They have the same syntax, but `c!` is eager (to a `Vec`), and
`i!` is lazy (to an `Iterator`).

Both macros may have any combination of `for` and `if` expressions,
but require a `for` expression to start. Ordering of loops and tests
is always from left to right, and variables are only available after
they are introduced, and in the final expression.

# Examples

## Nested `for` expressions
```rust
// Pythagorean triangle dimensions
let triangles: Vec<(i32, i32, i32)> =
    c![(x, y, z), for x in 1..11, for y in x..11, for z in y..11, if x * x + y * y == z * z];
assert_eq!(triangles, &[(3, 4, 5), (6, 8, 10)]);
```

## Nested macro usage, infinite series
```rust
// A very inefficient prime generator
let primes = i!(num, for num in 2..,
                     if i!(divisor, for divisor in 2..num,
                                    if num % divisor == 0).nth(0).is_none());
let primes_10: Vec<i32> = primes.take(10).collect();
assert_eq!(primes_10, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```