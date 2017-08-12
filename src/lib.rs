#![doc(html_logo_url = "https://raw.githubusercontent.com/mattgathu/cute/master/C!.png")]
//! Macros for Python-esque list and dictionary (hashmap) comprehensions in Rust.
//!
//! The `c!` and `i!` macros implements list and hashmap comprehensions
//! alongside list generators similar to those found in Python, allowing
//! for conditionals and nested comprehensions.
//!
//! # Python Syntax Example
//! ```python
//! even_squares = [x * x for x in range(10) if x % 2 == 0]
//! squares_dict = {key: key * key for key in range(10)}
//! even_squares_gen = (x * x for x in range(10) if x % 2 == 0)
//! ```
//!
//! # `c!` and `i!` Syntax Example
//! ```
//! #[macro_use(c, i)]
//! extern crate cute;
//!
//! # fn main() {
//! let even_squares = c![x * x, for x in 0..10, if x % 2 == 0];
//! let squares_hashmap = c!{key => key * key, for key in 0..10};
//! let even_squares_iter = i!(x * x, for x in 0..10, if x % 2 == 0);
//! # }
//! ```
//!
//! `c!` and `i!` have the comprehension's parts, comma-separated.
//! They have the same syntax, but `c!` is eager (to a `Vec`), and
//! `i!` is lazy (to an `Iterator`).
//!
//! Both macros may have any combination of `for` and `if` expressions,
//! but require a `for` expression to start. Ordering of loops and tests
//! is always from left to right, and variables are only available after
//! they are introduced, and in the final expression.
//!
//! # Examples
//!
//! ## Nested `for` expressions
//! ```
//! # #[macro_use(c)]
//! # extern crate cute;
//! # fn main() {
//! // Pythagorean triangle dimensions
//! let triangles: Vec<(i32, i32, i32)> =
//!     c![(x, y, z), for x in 1..11, for y in x..11, for z in y..11, if x * x + y * y == z * z];
//! assert_eq!(triangles, &[(3, 4, 5), (6, 8, 10)]);
//! # }
//! ```
//!
//! ## Nested macro usage, infinite series
//! ```
//! # #[macro_use(i)]
//! # extern crate cute;
//! # fn main() {
//! // A very inefficient prime generator
//! let primes = i!(num, for num in 2..,
//!                      if i!(divisor, for divisor in 2..num,
//!                                     if num % divisor == 0).nth(0).is_none());
//! let primes_10: Vec<i32> = primes.take(10).collect();
//! assert_eq!(primes_10, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//! # }
//! ```

#[macro_export]
macro_rules! c {
    // no tokens left
    (@iterate $cmd:expr => { } => { $i:pat, $iter:expr } => { $({ $cond:expr })* }) => {
        for $i in $iter {
            if !($($cond &&)* true) { continue }
            $cmd;
        }
    };
    // parsing "for if* .. for", found for
    (@iterate $cmd:expr => { for $new_i:pat in $new_iter:expr, $($t:tt)* } => { $i:pat, $iter:expr } => { $({ $cond:expr })* }) => {
        for $i in $iter {
            if !($($cond &&)* true) { continue }
            c!{@iterate $cmd => { $($t)* } => { $new_i, $new_iter } => { }};
        }
    };
    // parsing "for if* .. for", found if
    (@iterate $cmd:expr => { if $new_cond:expr, $($t:tt)* } => { $i:pat, $iter:expr } => { $({ $cond:expr })* }) => {
        c!{@iterate $cmd => { $($t)* } => { $i, $iter } => { $({ $cond })* { $new_cond } }};
    };
    // parsing, no context
    (@iterate $cmd:expr => { for $new_i:pat in $new_iter:expr, $($t:tt)* } => { } => { }) => {
        c!{@iterate $cmd => { $($t)* } => { $new_i, $new_iter } => { }};
    };
    // entry point - iterate to a vector
    ($exp:expr, $($t:tt)+) => {{
        let mut res = ::std::vec::Vec::new();
        c!{@iterate res.push($exp) => { $($t)*, } => { } => { }};
        res
    }};
    // entry point - iterate to a hashmap
    ($key:expr => $val:expr, $($t:tt)+) => {{
        let mut res = ::std::collections::HashMap::new();
        c!{@iterate res.insert($key, $val) => { $($t)*, } => { } => { }};
        res
    }};
}
#[macro_export]
macro_rules! i {
    // no tokens left, no conditions
    (@iterate $exp:expr => { } => { $i:pat, $iter:expr } => { }) => {
        $iter.into_iter()
            .map(move |$i| { $exp })
    };
    // no tokens left, has conditions
    (@iterate $exp:expr => { } => { $i:pat, $iter:expr } => { $({ $cond:expr })+ }) => {
        $iter.into_iter()
            .filter(move |&$i| $($cond &&)+ true)
            .map(move |$i| { $exp })
    };
    // parsing "for if* .. for", found for, no conditions
    (@iterate $exp:expr => { for $new_i:pat in $new_iter:expr, $($t:tt)* } => { $i:pat, $iter:expr } => { }) => {
        $iter.into_iter()
            .flat_map(move |$i| i!{@iterate $exp => { $($t)* } => { $new_i, $new_iter } => { }})
    };
    // parsing "for if* .. for", found for, has conditions
    (@iterate $exp:expr => { for $new_i:pat in $new_iter:expr, $($t:tt)* } => { $i:pat, $iter:expr } => { $({ $cond:expr })+ }) => {
        $iter.into_iter()
            .filter(move |&$i| $($cond &&)+ true)
            .flat_map(move |$i| i!{@iterate $exp => { $($t)* } => { $new_i, $new_iter } => { }})
    };
    // parsing "for if* .. for", found if
    (@iterate $exp:expr => { if $new_cond:expr, $($t:tt)* } => { $i:pat, $iter:expr } => { $({ $cond:expr })* }) => {
        i!{@iterate $exp => { $($t)* } => { $i, $iter } => { $({ $cond })* { $new_cond } }};
    };
    // parsing, no context
    (@iterate $exp:expr => { for $new_i:pat in $new_iter:expr, $($t:tt)* } => { } => { }) => {
        i!{@iterate $exp => { $($t)* } => { $new_i, $new_iter } => { }};
    };
    // entry point - iterate to an iterator
    ($exp:expr, $($t:tt)+) => {
        i!{@iterate $exp => { $($t)*, } => { } => { }}
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_eager() {
        let result = c![x, for x in 0..5];
        assert_eq!(result, &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn simple_lazy() {
        let result = i!(x, for x in 0..5);
        let result: Vec<i32> = result.collect();
        assert_eq!(result, &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn conditional_eager() {
        let result = c![x, for x in 0..5, if x % 2 == 0];
        assert_eq!(result, &[0, 2, 4]);
    }

    #[test]
    fn conditional_lazy() {
        let result = i!(x, for x in 0..5, if x % 2 == 0);
        let result: Vec<i32> = result.collect();
        assert_eq!(result, &[0, 2, 4]);
    }

    #[test]
    fn multiple_eager() {
        let result = c![x * y + z, for x in 0..5, for y in 0..5, if x % 2 == 0, if y % 3 == 1, for z in 0..2];
        assert_eq!(result, &[0, 1, 0, 1, 2, 3, 8, 9, 4, 5, 16, 17]);
    }

    #[test]
    fn multiple_lazy() {
        let result = i!(x * y + z, for x in 0..5, for y in 0..5, if x % 2 == 0, if y % 3 == 1, for z in 0..2);
        let result: Vec<i32> = result.collect();
        assert_eq!(result, &[0, 1, 0, 1, 2, 3, 8, 9, 4, 5, 16, 17]);
    }

    #[test]
    fn nested_eager() {
        let src = &[vec![1, 2, 3], vec![4, 5, 6]];
        let result = c![y, for x in src, for &y in x];
        assert_eq!(result, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn nested_lazy() {
        let src = &[vec![1, 2, 3], vec![4, 5, 6]];
        let result = i!(y, for x in src, for &y in x);
        let result: Vec<i32> = result.collect();
        assert_eq!(result, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn usage_nested_eager() {
        let result = c![x, for x in c![x + y, for x in 0..2, for y in 3..5]];
        assert_eq!(result, &[3, 4, 4, 5]);
    }

    #[test]
    fn usage_nested_lazy() {
        let result = i!(x, for x in i!(x + y, for x in 0..2, for y in 3..5));
        let result: Vec<i32> = result.collect();
        assert_eq!(result, &[3, 4, 4, 5]);
    }

    #[test]
    fn usage_mixed() {
        let result = c![x * x, for x in i!(y, for y in 0..5, if y % 2 == 0)];
        assert_eq!(result, &[0, 4, 16]);
    }

    #[test]
    fn apply_fn_eager() {
        fn square(x: i32) -> i32 { x * x }

        let result = c![square(x), for x in 0..5];
        assert_eq!(result, &[0, 1, 4, 9, 16]);
    }

    #[test]
    fn apply_fn_lazy() {
        fn square(x: i32) -> i32 { x * x }

        let result = i!(square(x), for x in 0..5);
        let result: Vec<i32> = result.collect();
        assert_eq!(result, &[0, 1, 4, 9, 16]);
    }
}