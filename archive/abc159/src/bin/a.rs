#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    n:i64,m:i64,
     }
    println!("{}", n * (n - 1) / 2 + m * (m - 1) / 2);
}
