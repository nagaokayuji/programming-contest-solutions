#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        k:usize,
        a:[i32;n],
    }
    for i in k..n {
        print!("{} ", if a[i - k] < a[i] { "Yes" } else { "No" });
    }
}
