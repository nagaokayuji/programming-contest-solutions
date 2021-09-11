#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        k:i64,x:i64,
    }
    // x+k-1
    // x-k+1
    for p in x - k + 1..=x + k - 1 {
        println!("{}", p);
    }
}
