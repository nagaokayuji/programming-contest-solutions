#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        a:i64,
        mut p:i64,
    }
    p += a * 3;
    println!("{}", p / 2);
}
