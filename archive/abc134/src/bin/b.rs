#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,d:usize,
    }
    // 2*d + 1 本いける
    let waru = 2 * d + 1;
    println!("{}", (n + waru - 1) / waru);
}
