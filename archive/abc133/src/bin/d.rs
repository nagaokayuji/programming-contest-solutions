#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a:[i64;n],
    }

    let sm = a.iter().fold(0, |sum, x| sum + x);
    let mut anss = vec![0; n];
    anss[0] = sm;
    for i in 1..n {
        if i % 2 == 1 {
            anss[0] -= 2 * a[i];
        }
    }
    println!("{}", anss[0]);
    for i in 1..n {
        anss[i] = 2 * a[i - 1] - anss[i - 1];
        println!("{}", anss[i]);
    }
}
