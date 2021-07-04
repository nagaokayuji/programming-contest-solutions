#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[Usize1;n-1],
    }
    let mut v = vec![0; n + 3];
    for &x in a.iter() {
        v[x] += 1;
    }
    for i in 0..n {
        println!("{}", v[i]);
    }
}
