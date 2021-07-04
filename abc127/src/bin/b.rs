#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    r:usize,d:usize,x:usize,
     }

    let mut prev = x;
    for _ in 0..10 {
        prev = r * prev - d;
        println!("{}", prev);
    }
}
