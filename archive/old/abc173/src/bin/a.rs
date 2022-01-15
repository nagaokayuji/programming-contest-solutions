#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        a :i64,
    }
    println!("{}", if a % 1000 != 0 { 1000 - a % 1000 } else { 0 });
}
