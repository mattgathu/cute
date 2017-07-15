// Macro for python-esque list comprehensions in Rust
// 
// 
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
