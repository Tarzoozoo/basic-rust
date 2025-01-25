pub fn print_debug<T: std::fmt::Debug>(value: T) {

    println!("----------------------------------> Debug");
    println!("{:?}", value);
}

pub fn duplicate<T: Clone>(value: T) -> (T, T) {

    println!("----------------------------------> Clone");
    (value.clone(), value)
}

pub fn are_equal<T: PartialEq>(a: T, b: T) -> bool {

    println!("----------------------------------> PartialEq");
    a == b
}


pub fn compare_values<T: Ord>(a: T, b: T) -> &'static str {

    println!("----------------------------------> Ord");
    if a < b {
        "a is less than b"
    } else if a > b {
        "a is greater than b"
    } else {
        "a is equal to b"
    }
}

use std::collections::HashSet;
use std::hash::Hash;

pub fn unique_items<T: Eq + Hash>(items: Vec<T>) -> HashSet<T> {

    println!("----------------------------------> Hash");
    items.into_iter().collect()
}


use std::ops::Add;

pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {

    println!("----------------------------------> Add");
    a + b
}


pub fn sum_values<T: std::iter::Sum>(values: Vec<T>) -> T {

    // 
    println!("----------------------------------> Iterator");
    values.into_iter().sum()
}