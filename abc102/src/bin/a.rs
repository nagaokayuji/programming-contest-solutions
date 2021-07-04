#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:i64,
    }
    if n & 1 == 1 {
        println!("{}", n * 2);
    } else {
        println!("{}", n);
    }
}
