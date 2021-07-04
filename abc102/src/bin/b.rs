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
    a.sort();
    println!("{}", a[n - 1] - a[0]);
}
